use std::{marker::PhantomData, ptr::NonNull};

/// Buffer implementation using a heap-allocated continuous array.
pub struct HeapBuffer<T> {
    ptr: NonNull<T>,
    cap: usize,
    _m: PhantomData<T>,
}
