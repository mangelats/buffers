use std::{
    alloc::{Allocator, Global, Layout},
    marker::PhantomData,
    ptr::NonNull,
};

use crate::interface::{resize_error::ResizeError, Buffer};

/// Similar buffer to HeapBuffer but it uses Allocators instead
pub struct AllocatorBuffer<T, A: Allocator = Global> {
    ptr: NonNull<T>,
    cap: usize,
    alloc: A,
    _marker: PhantomData<T>,
}

impl<T, A: Allocator + Default> AllocatorBuffer<T, A> {
    /// Makes a new buffer by default-constructing the allocator
    pub fn new() -> Self {
        Self::with_allocator(Default::default())
    }
}

impl<T, A: Allocator> AllocatorBuffer<T, A> {
    /// Make an empty `AllocatorBuffer` given an allocator
    pub fn with_allocator(alloc: A) -> Self {
        Self {
            ptr: NonNull::dangling(),
            cap: 0,
            alloc,
            _marker: PhantomData,
        }
    }

    /// Get a constant pointer to the specified index
    pub unsafe fn ptr(&self, index: usize) -> *const T {
        self.ptr.as_ptr().add(index)
    }

    /// Get a mutable pointer to the specified index
    pub unsafe fn mut_ptr(&mut self, index: usize) -> *mut T {
        self.ptr.as_ptr().add(index)
    }

    /// Internal function that sets the capacity and raw buffer pointer
    fn update_buffer(&mut self, ptr: NonNull<T>, cap: usize) {
        self.cap = cap;
        self.ptr = ptr;
    }
}

impl<T, A: Allocator> Buffer<T> for AllocatorBuffer<T, A> {
    fn capacity(&self) -> usize {
        self.cap
    }

    unsafe fn read_value(&self, index: usize) -> T {
        std::ptr::read(self.ptr(index))
    }

    unsafe fn write_value(&mut self, index: usize, value: T) {
        std::ptr::write(self.mut_ptr(index), value)
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        std::ptr::drop_in_place(self.mut_ptr(index));
    }
}

/// Internal utility function.
///
/// Tries to allocate a new array of a given size on the heap using the stable functions.
unsafe fn try_allocate<T, A: Allocator>(alloc: &A, size: usize) -> Result<NonNull<T>, ResizeError> {
    let new_layout = Layout::array::<T>(size)?;

    let new_ptr = alloc
        .allocate(new_layout)
        .map_err(|_| ResizeError::OutOfMemory)?;

    Ok(new_ptr.cast())
}

/// Internal utility function.
///
/// Tries to reallocate an array to a given size on the heap using the stable functions.
unsafe fn try_grow<T, A: Allocator>(
    alloc: &A,
    old_ptr: NonNull<T>,
    old_size: usize,
    new_size: usize,
) -> Result<NonNull<T>, ResizeError> {
    let old_layout = Layout::array::<T>(old_size)?;
    let new_layout = Layout::array::<T>(new_size)?;

    let new_ptr = alloc
        .grow(old_ptr.cast(), old_layout, new_layout)
        .map_err(|_| ResizeError::OutOfMemory)?;

    Ok(new_ptr.cast())
}

/// Internal utility function.
///
/// Tries to reallocate an array to a given size on the heap using the stable functions.
unsafe fn try_shrink<T, A: Allocator>(
    alloc: &A,
    old_ptr: NonNull<T>,
    old_size: usize,
    new_size: usize,
) -> Result<NonNull<T>, ResizeError> {
    let old_layout = Layout::array::<T>(old_size)?;
    let new_layout = Layout::array::<T>(new_size)?;

    let new_ptr = alloc
        .shrink(old_ptr.cast(), old_layout, new_layout)
        .map_err(|_| ResizeError::OutOfMemory)?;

    Ok(new_ptr.cast())
}

/// Internal utility function.
///
/// Tries to reallocate an array to a given size on the heap using the stable functions.
unsafe fn try_deallocate<T, A: Allocator>(
    alloc: &A,
    old_ptr: NonNull<T>,
    old_size: usize,
) -> Result<(), ResizeError> {
    let old_layout = Layout::array::<T>(old_size)?;
    alloc.deallocate(old_ptr.cast(), old_layout);
    Ok(())
}
