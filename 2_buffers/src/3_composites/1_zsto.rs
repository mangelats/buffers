use std::marker::PhantomData;

use crate::interface::Buffer;

use super::conditional::Selector;

/// Composite buffer that automatically uses a ZstBuffer when T is a ZST.
pub struct ZstOptBuffer<T, B: Buffer<T>> {
    child: B,
    _m: PhantomData<T>,
}

struct ZstSelector<T>(PhantomData<T>);
impl<T> Selector for ZstSelector<T> {
    const SELECT_A: bool = std::mem::size_of::<T>() == 0;
}
