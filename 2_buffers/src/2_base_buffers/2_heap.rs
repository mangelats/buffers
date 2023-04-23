use std::{
    alloc::Layout,
    marker::PhantomData,
    ptr::{self, NonNull},
};

use crate::interface::{
    continuous_memory::ContinuousMemoryBuffer, resize_error::ResizeError, Buffer,
};

/// Buffer implementation using a heap-allocated continuous array.
pub struct HeapBuffer<T> {
    buffer_start: NonNull<T>,
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
            buffer_start: NonNull::dangling(),
            cap: 0,
            _marker: PhantomData,
        }
    }

    /// Get a constant pointer to the specified index
    pub unsafe fn ptr(&self, index: usize) -> *const T {
        self.buffer_start.as_ptr().add(index)
    }

    /// Get a mutable pointer to the specified index
    pub unsafe fn mut_ptr(&mut self, index: usize) -> *mut T {
        self.buffer_start.as_ptr().add(index)
    }

    /// Internal function that allocates a new array into the heap
    ///
    /// # Safety
    /// It can only be called when there is no array allocated (capacity is 0)
    unsafe fn allocate_array(&mut self, target: usize) -> Result<(), ResizeError> {
        debug_assert!(self.cap == 0);
        let ptr = try_array_alloc(target)?;
        self.update_buffer(ptr, target);
        Ok(())
    }

    /// Internal function that resizes the array in the heap
    ///
    /// # Safety
    /// There needs to be an array already heap allocated. Target should be bigger than 0.
    unsafe fn resize_array(&mut self, target: usize) -> Result<(), ResizeError> {
        debug_assert!(target > 0);
        let ptr = try_array_realloc(self.buffer_start, self.cap, target)?;
        self.update_buffer(ptr, target);
        Ok(())
    }

    /// Internal function that deallocates the heap allocated array
    ///
    /// # Safety
    /// There needs to be an array heap allocated
    unsafe fn deallocate_array(&mut self) -> Result<(), ResizeError> {
        deallocate(self.buffer_start, self.cap)?;
        self.update_buffer(NonNull::dangling(), 0);
        Ok(())
    }

    /// Internal function that sets the capacity and raw buffer pointer
    fn update_buffer(&mut self, ptr: NonNull<T>, cap: usize) {
        self.cap = cap;
        self.buffer_start = ptr;
    }
}

impl<T> ContinuousMemoryBuffer for HeapBuffer<T> {
    unsafe fn ptr(&self, index: usize) -> *const T {
        debug_assert!(index < self.capacity());
        todo!()
    }

    unsafe fn mut_ptr(&mut self, index: usize) -> *mut T {
        debug_assert!(index < self.capacity());
        todo!()
    }
}

impl<T> Buffer for HeapBuffer<T> {
    type Element = T;

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
            self.allocate_array(target)
        } else {
            self.resize_array(target)
        }
    }

    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError> {
        if target == 0 {
            self.deallocate_array()
        } else {
            self.resize_array(target)
        }
    }
}

impl<T> Default for HeapBuffer<T> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<#[may_dangle] T> Drop for HeapBuffer<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            // SAFETY: At this point all content should have been dropped
            unsafe {
                let _ = self.deallocate_array();
            }
        }
    }
}

/// Tries to allocate an array of a given size on the heap
///
/// # Safety
/// size must be bigger than zero.
unsafe fn try_array_alloc<T>(size: usize) -> Result<NonNull<T>, ResizeError> {
    debug_assert!(size > 0);
    let layout = Layout::array::<T>(size)?;

    let ptr = std::alloc::alloc(layout);
    let ptr = ptr as *mut T;
    NonNull::new(ptr).ok_or(ResizeError::OutOfMemory)
}

/// Tries to reallocate an existing heap array (growing or shrinking)
///
/// # Safety
/// new_size must be different than old_size.
/// new_size must be bigger than zero.
unsafe fn try_array_realloc<T>(
    old_ptr: NonNull<T>,
    old_size: usize,
    new_size: usize,
) -> Result<NonNull<T>, ResizeError> {
    debug_assert!(new_size > 0);
    debug_assert!(old_size != new_size);
    let old_layout = Layout::array::<T>(old_size)?;
    let new_layout = Layout::array::<T>(new_size)?;

    let old_ptr = old_ptr.as_ptr() as *mut u8;

    let new_ptr = std::alloc::realloc(old_ptr, old_layout, new_layout.size());
    let new_ptr = new_ptr as *mut T;

    NonNull::new(new_ptr).ok_or(ResizeError::OutOfMemory)
}

/// Deallocates an array
unsafe fn deallocate<T>(ptr: NonNull<T>, size: usize) -> Result<(), ResizeError> {
    debug_assert!(size > 0);
    let layout = Layout::array::<T>(size)?;
    let ptr = ptr.as_ptr();
    let ptr = ptr as *mut u8;
    std::alloc::dealloc(ptr, layout);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_grow_from_default() {
        const TARGET: usize = 1;

        let mut buffer = HeapBuffer::<i32>::new();

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

        let mut buffer = HeapBuffer::<i32>::new();

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

        let mut buffer = HeapBuffer::<i32>::new();

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

        let mut buffer = HeapBuffer::<i32>::new();

        // SAFETY: 0 == TARGET2 < TARGET1
        unsafe {
            buffer.try_grow(TARGET1).unwrap();
            buffer.try_shrink(TARGET2).unwrap();
        }

        assert!(buffer.capacity() < TARGET1);
        assert!(buffer.capacity() == TARGET2);
    }
}
