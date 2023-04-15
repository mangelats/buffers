use crate::{base_buffers::inline::InlineBuffer, interface::Buffer};

pub struct SvoBuffer<T, B: Buffer<T>, const SMALL_SIZE: usize> {
    small: InlineBuffer<T, SMALL_SIZE>,
    big: B,
}

impl<T, B: Buffer<T>, const SMALL_SIZE: usize> Buffer<T> for SvoBuffer<T, B, SMALL_SIZE> {
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
