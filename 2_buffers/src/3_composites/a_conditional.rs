use std::marker::PhantomData;

use crate::interface::Buffer;

pub struct ConditionalBuffer<T, A: Buffer<T>, B: Buffer<T>, const SELECT_A: bool> {
    a: A,
    b: B,
    _m: PhantomData<T>,
}
