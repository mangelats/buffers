use std::ops::RangeBounds;

use crate::{
    base_buffers::inline::InlineBuffer,
    interface::{
        continuous_memory::ContinuousMemoryBuffer, ptrs::PtrBuffer, refs::RefBuffer,
        resize_error::ResizeError, Buffer,
    },
};

use super::either::EitherBuffer;

/// Buffer composite that adds small vector optimization (SVO) to a given buffer.
///
/// This means that it can work with both an inline buffer (which is usually left on the stack)
/// but can automatically grow into a bigger buffer (usually a heap-allocated one).
pub struct SvoBuffer<T, const SMALL_SIZE: usize, B>
where
    B: Buffer<Element = T> + Default,
{
    inner: EitherBuffer<T, InlineBuffer<T, SMALL_SIZE>, B>,
}

impl<T, const SMALL_SIZE: usize, B> SvoBuffer<T, SMALL_SIZE, B>
where
    B: Buffer<Element = T> + Default,
{
    /// Creates a new empty buffer
    pub fn new() -> Self {
        Default::default()
    }

    /// Internal only.
    ///
    /// Move all data from the small vector into the big one
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

impl<T, const SMALL_SIZE: usize, B> Default for SvoBuffer<T, SMALL_SIZE, B>
where
    B: Buffer<Element = T> + Default,
{
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<T, const SMALL_SIZE: usize, B> Buffer for SvoBuffer<T, SMALL_SIZE, B>
where
    B: Buffer<Element = T> + Default,
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

    unsafe fn manually_drop_range<R: RangeBounds<usize>>(&mut self, values_range: R) {
        self.inner.manually_drop_range(values_range)
    }
    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        match self.inner {
            EitherBuffer::First(_) => self.move_into_big(target),
            EitherBuffer::Second(ref mut buf) => buf.try_grow(target),
            EitherBuffer::_InternalMarker(_) => unreachable!(),
        }
    }
    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError> {
        match self.inner {
            EitherBuffer::First(_) => Ok(()),
            EitherBuffer::Second(ref mut buf) => buf.try_shrink(target),
            EitherBuffer::_InternalMarker(_) => unreachable!(),
        }
    }
}

impl<T, const SMALL_SIZE: usize, B> PtrBuffer for SvoBuffer<T, SMALL_SIZE, B>
where
    B: Buffer<Element = T>
        + Default
        + PtrBuffer<ConstantPointer = *const T, MutablePointer = *mut T>,
{
    type ConstantPointer = *const T;
    type MutablePointer = *mut T;

    unsafe fn ptr(&self, index: usize) -> *const Self::Element {
        self.inner.ptr(index)
    }

    unsafe fn mut_ptr(&mut self, index: usize) -> *mut Self::Element {
        self.inner.mut_ptr(index)
    }
}

impl<T, const SMALL_SIZE: usize, B> RefBuffer for SvoBuffer<T, SMALL_SIZE, B>
where
    B: Buffer<Element = T> + Default,
    for<'a> B: RefBuffer<ConstantReference<'a> = &'a T, MutableReference<'a> = &'a mut T> + 'a,
{
    type ConstantReference<'a> = &'a T
    where
        Self: 'a;

    type MutableReference<'a> = &'a mut T
    where
        Self: 'a;

    unsafe fn index(&self, index: usize) -> Self::ConstantReference<'_> {
        // For some reason the borrow checker can't check `self.inner.index(index)`
        match self.inner {
            EitherBuffer::First(ref b) => RefBuffer::index(b, index),
            EitherBuffer::Second(ref b) => RefBuffer::index(b, index),
            EitherBuffer::_InternalMarker(_) => unreachable!(),
        }
    }

    unsafe fn mut_index(&mut self, index: usize) -> Self::MutableReference<'_> {
        // For some reason the borrow checker can't check `self.inner.mut_index(index)`
        match self.inner {
            EitherBuffer::First(ref mut b) => RefBuffer::mut_index(b, index),
            EitherBuffer::Second(ref mut b) => RefBuffer::mut_index(b, index),
            EitherBuffer::_InternalMarker(_) => unreachable!(),
        }
    }
}

impl<T, const SMALL_SIZE: usize, B> ContinuousMemoryBuffer for SvoBuffer<T, SMALL_SIZE, B> where
    B: Buffer<Element = T> + Default + ContinuousMemoryBuffer
{
}

#[cfg(test)]
mod tests {
    use crate::base_buffers::heap::HeapBuffer;

    use super::*;

    #[test]
    fn should_be_able_to_grow() {
        let mut buffer: SvoBuffer<u32, 1, HeapBuffer<u32>> = Default::default();
        assert_eq!(buffer.capacity(), 1);
        unsafe { buffer.try_grow(32) }.expect("Should be able to grow");
        assert!(buffer.capacity() >= 32)
    }

    #[test]
    fn should_move_elements_when_growing() {
        let mut buffer: SvoBuffer<u32, 1, HeapBuffer<u32>> = Default::default();
        unsafe {
            buffer.write_value(0, 123);
            buffer.try_grow(32).expect("Should be able to grow");
            assert_eq!(buffer.read_value(0), 123);
        }
    }
}
