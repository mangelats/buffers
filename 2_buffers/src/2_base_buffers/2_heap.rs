use std::{
    alloc::Layout,
    marker::PhantomData,
    ptr::{self, NonNull},
};

use crate::interface::{resize_result::ResizeError, Buffer};

/// Buffer implementation using a heap-allocated continuous array.
pub struct HeapBuffer<T> {
    ptr: NonNull<T>,
    cap: usize,
    _marker: PhantomData<T>,
}

impl<T> HeapBuffer<T> {
    /// Makes a new default-sized `HeapBuffer`.
    ///
    /// ```
    /// # use buffers::base_buffers::heap::HeapBuffer;
    /// let buffer = HeapBuffer::<u32>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            cap: 0,
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

    /// Internal function that allocates a new array into the heap
    unsafe fn allocate_array_unchecked(&mut self, target: usize) -> Result<(), ResizeError> {
        debug_assert!(self.cap == 0);
        let ptr = try_array_alloc(target)?;
        self.update_buffer(ptr, target);
        Ok(())
    }

    unsafe fn reallocate_array_unchecked(&mut self, target: usize) -> Result<(), ResizeError> {
        let ptr = try_array_realloc(self.ptr, self.cap, target)?;
        self.update_buffer(ptr, target);
        Ok(())
    }

    /// Internal function that sets the capacity and raw buffer pointer
    fn update_buffer(&mut self, ptr: NonNull<T>, cap: usize) {
        self.cap = cap;
        self.ptr = ptr;
    }
}

impl<T> Buffer<T> for HeapBuffer<T> {
    fn capacity(&self) -> usize {
        self.cap
    }

    unsafe fn read_value(&self, index: usize) -> T {
        ptr::read(self.ptr(index))
    }

    unsafe fn write_value(&mut self, index: usize, value: T) {
        ptr::write(self.mut_ptr(index), value)
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        ptr::drop_in_place(self.mut_ptr(index));
    }

    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        if self.cap == 0 {
            self.allocate_array_unchecked(target)
        } else {
            Ok(())
        }
    }
}

impl<T> Default for HeapBuffer<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Tries to allocate an array of a given size on the heap
unsafe fn try_array_alloc<T>(size: usize) -> Result<NonNull<T>, ResizeError> {
    debug_assert!(size > 0);
    let layout = Layout::array::<T>(size)?;

    let ptr = std::alloc::alloc(layout);
    let ptr = ptr as *mut T;
    NonNull::new(ptr).ok_or(ResizeError::OutOfMemory)
}

/// Tries to reallocate an existing heap array (growing or shrinking)
unsafe fn try_array_realloc<T>(
    old_ptr: NonNull<T>,
    old_size: usize,
    new_size: usize,
) -> Result<NonNull<T>, ResizeError> {
    debug_assert!(old_size != new_size);
    let old_layout = Layout::array::<T>(old_size)?;
    let new_layout = Layout::array::<T>(new_size)?;

    let old_ptr = old_ptr.as_ptr() as *mut u8;

    let new_ptr = std::alloc::realloc(old_ptr, old_layout, new_layout.size());
    let new_ptr = new_ptr as *mut T;

    NonNull::new(new_ptr).ok_or(ResizeError::OutOfMemory)
}
