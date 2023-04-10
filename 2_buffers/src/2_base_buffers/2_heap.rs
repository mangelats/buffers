use std::{marker::PhantomData, ptr::NonNull};

pub struct HeapBuffer<T> {
    ptr: NonNull<T>,
    cap: usize,
    _m: PhantomData<T>,
}
