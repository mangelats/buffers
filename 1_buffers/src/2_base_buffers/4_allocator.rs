use std::{
    alloc::{Allocator, Global, Layout},
    marker::PhantomData,
    ptr::NonNull,
};

use crate::interface::{
    continuous_memory::ContinuousMemoryBuffer, ptrs::PtrBuffer, refs::DefaultRefBuffer,
    resize_error::ResizeError, Buffer,
};

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

    /// Internal function that sets the capacity and raw buffer pointer
    fn update_buffer(&mut self, ptr: NonNull<T>, cap: usize) {
        self.cap = cap;
        self.ptr = ptr;
    }
}

impl<T, A: Allocator> Buffer for AllocatorBuffer<T, A> {
    type Element = T;

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

    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        let ptr = if self.cap > 0 {
            try_grow(&self.alloc, self.ptr, self.cap, target)
        } else {
            try_allocate(&self.alloc, target)
        }?;
        self.update_buffer(ptr, target);
        Ok(())
    }

    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError> {
        if target == 0 {
            try_deallocate(&self.alloc, self.ptr, self.cap)?;
            self.update_buffer(NonNull::dangling(), 0);
            Ok(())
        } else {
            let ptr = try_shrink(&self.alloc, self.ptr, self.cap, target)?;
            self.update_buffer(ptr, target);
            Ok(())
        }
    }
}

impl<T, A: Allocator> PtrBuffer for AllocatorBuffer<T, A> {
    type ConstantPointer = *const T;
    type MutablePointer = *mut T;

    unsafe fn ptr(&self, index: usize) -> *const Self::Element {
        self.ptr.as_ptr().add(index)
    }

    unsafe fn mut_ptr(&mut self, index: usize) -> *mut Self::Element {
        self.ptr.as_ptr().add(index)
    }
}
impl<T, A: Allocator> ContinuousMemoryBuffer for AllocatorBuffer<T, A> {}
impl<T, A: Allocator> DefaultRefBuffer for AllocatorBuffer<T, A> {}

impl<T, A: Allocator + Default> Default for AllocatorBuffer<T, A> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<#[may_dangle] T, A: Allocator> Drop for AllocatorBuffer<T, A> {
    fn drop(&mut self) {
        if self.cap != 0 {
            // SAFETY: At this point all content should have been dropped
            unsafe {
                let _ = try_deallocate(&self.alloc, self.ptr, self.cap);
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_grow_from_default() {
        const TARGET: usize = 1;

        let mut buffer = AllocatorBuffer::<i32, Global>::new();

        // SAFETY: 0 < TARGET
        unsafe {
            buffer.try_grow(TARGET).unwrap();
        }

        assert!(buffer.capacity() >= TARGET);
    }

    #[test]
    fn can_grow_twice() {
        const TARGET1: usize = 1;
        const TARGET2: usize = 10;

        let mut buffer = AllocatorBuffer::<i32, Global>::new();

        // SAFETY: 0 < TARGET1 < TARGET2
        unsafe {
            buffer.try_grow(TARGET1).unwrap();
            buffer.try_grow(TARGET2).unwrap();
        }

        assert!(buffer.capacity() >= TARGET2);
    }

    #[test]
    fn can_shrink() {
        const TARGET1: usize = 64;
        const TARGET2: usize = 1;

        let mut buffer = AllocatorBuffer::<i32, Global>::new();

        // SAFETY: 0 < TARGET2 < TARGET1
        unsafe {
            buffer.try_grow(TARGET1).unwrap();
            buffer.try_shrink(TARGET2).unwrap();
        }

        assert!(buffer.capacity() < TARGET1);
        assert!(buffer.capacity() >= TARGET2);
    }

    #[test]
    fn can_shrink_to_nothing() {
        const TARGET1: usize = 64;
        const TARGET2: usize = 0;

        let mut buffer = AllocatorBuffer::<i32, Global>::new();

        // SAFETY: 0 == TARGET2 < TARGET1
        unsafe {
            buffer.try_grow(TARGET1).unwrap();
            buffer.try_shrink(TARGET2).unwrap();
        }

        assert!(buffer.capacity() < TARGET1);
        assert!(buffer.capacity() == TARGET2);
    }
}
