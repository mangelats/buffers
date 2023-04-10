use std::{marker::PhantomData, ptr::NonNull};

use crate::interface::Buffer;

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
    ///
    /// ```
    /// ```
    pub unsafe fn ptr(&self, index: usize) -> *const T {
        self.ptr.as_ptr().add(index)
    }

    unsafe fn mut_ptr(&mut self, index: usize) -> *mut T {
        self.ptr.as_ptr().add(index)
    }
}

impl<T> Buffer<T> for HeapBuffer<T> {
    fn capacity(&self) -> usize {
        self.cap
    }

    unsafe fn read_value(&self, index: usize) -> T {
        todo!()
    }

    unsafe fn write_value(&mut self, index: usize, value: T) {
        todo!()
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        todo!()
    }
}

impl<T> Default for HeapBuffer<T> {
    fn default() -> Self {
        Self::new()
    }
}
