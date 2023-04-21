use std::ops::Range;

use crate::{
    base_buffers::inline::InlineBuffer,
    interface::{resize_error::ResizeError, Buffer},
};

use super::either::EitherBuffer;

/// Buffer composite that adds small vector optimization (SVO) to a given buffer.
pub struct SvoBuffer<T, B: Buffer<Element = T> + Default, const SMALL_SIZE: usize> {
    inner: EitherBuffer<T, InlineBuffer<T, SMALL_SIZE>, B>,
}

impl<T, B: Buffer<Element = T> + Default, const SMALL_SIZE: usize> SvoBuffer<T, B, SMALL_SIZE> {
    /// Creates a new empty buffer
    pub fn new() -> Self {
        Default::default()
    }

    unsafe fn move_into_big(&mut self, target: usize) -> Result<(), ResizeError> {
        let EitherBuffer::First(ref current_buf) = self.inner else {
            // SAFETY: This is only called when we grow from small to big.
            // This means that we always have an inline buffer at this point
            unreachable!()
        };

        let mut new_buf: B = Default::default();
        if new_buf.capacity() < target {
            new_buf.try_grow(target)?;
        }

        // TODO: either detect or force B to have a continuous array so we can use
        // `ptr.copy_to_nonoverlapping` instead of copying element by element
        for index in 0..current_buf.capacity() {
            new_buf.write_value(index, current_buf.read_value(index))
        }

        self.inner = EitherBuffer::Second(new_buf);
        Ok(())
    }
}

impl<T, B: Buffer<Element = T> + Default, const SMALL_SIZE: usize> Default
    for SvoBuffer<T, B, SMALL_SIZE>
{
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<T, B: Buffer<Element = T> + Default, const SMALL_SIZE: usize> Buffer
    for SvoBuffer<T, B, SMALL_SIZE>
{
    type Element = T;

    fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    unsafe fn read_value(&self, index: usize) -> T {
        self.inner.read_value(index)
    }

    unsafe fn write_value(&mut self, index: usize, value: T) {
        self.inner.write_value(index, value)
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        self.inner.manually_drop(index)
    }

    unsafe fn manually_drop_range(&mut self, values_range: Range<usize>) {
        self.inner.manually_drop_range(values_range)
    }
    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        match self.inner {
            EitherBuffer::First(_) => self.move_into_big(target),
            EitherBuffer::Second(ref mut buf) => buf.try_grow(target),
            EitherBuffer::_InternalMarker(_, _) => unreachable!(),
        }
    }
    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError> {
        match self.inner {
            EitherBuffer::First(_) => Ok(()),
            EitherBuffer::Second(ref mut buf) => buf.try_shrink(target),
            EitherBuffer::_InternalMarker(_, _) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base_buffers::heap::HeapBuffer;

    use super::*;

    #[test]
    fn should_be_able_to_grow() {
        let mut buffer: SvoBuffer<u32, HeapBuffer<u32>, 1> = Default::default();
        assert_eq!(buffer.capacity(), 1);
        unsafe { buffer.try_grow(32) }.expect("Should be able to grow");
        assert!(buffer.capacity() >= 32)
    }

    #[test]
    fn should_move_elements_when_growing() {
        let mut buffer: SvoBuffer<u32, HeapBuffer<u32>, 1> = Default::default();
        unsafe {
            buffer.write_value(0, 123);
            buffer.try_grow(32).expect("Should be able to grow");
            assert_eq!(buffer.read_value(0), 123);
        }
    }
}
