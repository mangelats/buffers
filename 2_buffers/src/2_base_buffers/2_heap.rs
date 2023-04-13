use std::{
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

    unsafe fn allocate_array_unchecked(&mut self, target: usize) -> Result<(), ResizeError> {
        Ok(())
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
        debug_assert!(target > self.cap);
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
unsafe fn try_array_alloc<T>(size: usize) -> Result<(), ResizeError> {
    Ok(())
}
