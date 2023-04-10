use std::{marker::PhantomData, ptr::NonNull};

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
}

impl<T> Default for HeapBuffer<T> {
    fn default() -> Self {
        Self::new()
    }
}
