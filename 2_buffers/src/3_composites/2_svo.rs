use std::ops::Range;

use crate::{
    base_buffers::inline::InlineBuffer,
    interface::{resize_error::ResizeError, Buffer},
};

use super::either::EitherBuffer;

/// Buffer composite that adds small vector optimization (SVO) to a given buffer.
pub struct SvoBuffer<T, B: Buffer<T> + Default, const SMALL_SIZE: usize> {
    inner: EitherBuffer<T, InlineBuffer<T, SMALL_SIZE>, B>,
}

impl<T, B: Buffer<T> + Default, const SMALL_SIZE: usize> SvoBuffer<T, B, SMALL_SIZE> {
    /// Creates a new empty buffer
    pub fn new() -> Self {
        Default::default()
    }

    unsafe fn move_into_big(&mut self, target: usize) -> Result<(), ResizeError> {
        let EitherBuffer::First(ref current_buf) = self.inner else {
            unreachable!() // SAFETY: This is only called when we grow from small to big. So it's always first
        };

        let mut new_buf: B = Default::default();
        new_buf.try_grow(target)?;

        for index in 0..current_buf.capacity() {
            new_buf.write_value(index, current_buf.read_value(index))
        }

        self.inner = EitherBuffer::Second(new_buf);
        Ok(())
    }
}

impl<T, B: Buffer<T> + Default, const SMALL_SIZE: usize> Default for SvoBuffer<T, B, SMALL_SIZE> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<T, B: Buffer<T> + Default, const SMALL_SIZE: usize> Buffer<T> for SvoBuffer<T, B, SMALL_SIZE> {
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
