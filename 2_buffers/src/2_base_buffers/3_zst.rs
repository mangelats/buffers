use std::marker::PhantomData;

use crate::interface::Buffer;

pub struct ZstBuffer<T> {
    _m: PhantomData<T>,
}

impl<T> Buffer<T> for ZstBuffer<T> {
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
