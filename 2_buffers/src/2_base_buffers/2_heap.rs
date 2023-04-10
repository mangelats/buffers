use std::{marker::PhantomData, ptr::NonNull};

/// Buffer implementation using a heap-allocated continuous array.
pub struct HeapBuffer<T> {
    ptr: NonNull<T>,
    cap: usize,
    _m: PhantomData<T>,
}

impl<T> HeapBuffer<T> {
    /// Makes a new default-sized `HeapBuffer`
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            cap: 0,
            _m: PhantomData,
        }
    }
}
