use std::marker::PhantomData;

use crate::interface::Buffer;

pub struct ConditionalBuffer<T, A: Buffer<T>, B: Buffer<T>, const SELECT_A: bool> {
    a: A,
    b: B,
    _m: PhantomData<T>,
}

impl<T, A: Buffer<T>, B: Buffer<T>, const SELECT_A: bool> Buffer<T>
    for ConditionalBuffer<T, A, B, SELECT_A>
{
    fn capacity(&self) -> usize {
        todo!()
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
