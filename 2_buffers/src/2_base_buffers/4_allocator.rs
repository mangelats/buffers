use std::{
    alloc::{Allocator, Global},
    marker::PhantomData,
    ptr::NonNull,
};

/// Similar buffer to HeapBuffer but it uses Allocators instead
pub struct AllocatorBuffer<T, A: Allocator> {
    ptr: NonNull<T>,
    cap: usize,
    alloc: A,
    _marker: PhantomData<T>,
}

impl<T, A: Allocator + Default> AllocatorBuffer<T, A> {
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
}
