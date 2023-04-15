use std::marker::PhantomData;

use crate::interface::Buffer;

/// Composite buffer that automatically uses a ZstBuffer when T is a ZST.
pub struct ZstOptBuffer<T, B: Buffer<T>> {
    child: B,
    _m: PhantomData<T>,
}
