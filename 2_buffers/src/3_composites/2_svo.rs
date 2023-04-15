use crate::{base_buffers::inline::InlineBuffer, interface::Buffer};

use super::either::EitherBuffer;

pub struct SvoBuffer<T, B: Buffer<T>, const SMALL_SIZE: usize> {
    inner: EitherBuffer<T, InlineBuffer<T, SMALL_SIZE>, B>,
}

impl<T, B: Buffer<T>, const SMALL_SIZE: usize> Buffer<T> for SvoBuffer<T, B, SMALL_SIZE> {
    fn capacity(&self) -> usize {
        self.inner.capacity()
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
