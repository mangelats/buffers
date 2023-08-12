use std::ops::RangeBounds;

use crate::interface::{
    continuous_memory::ContinuousMemoryBuffer, ptrs::PtrBuffer, refs::RefBuffer,
    resize_error::ResizeError, Buffer,
};

/// Utility buffer that may contain one of two buffers.
///
/// It's a Buffer itself, forwarding the requests to the currently selected.
pub enum EitherBuffer<A, B>
where
    A: Buffer,
    B: Buffer<Element = A::Element>,
{
    /// First option (A buffer)
    First(A),
    /// Second option (B buffer)
    Second(B),
}

impl<A, B> Default for EitherBuffer<A, B>
where
    A: Buffer + Default,
    B: Buffer<Element = A::Element>,
{
    fn default() -> Self {
        Self::First(Default::default())
    }
}

impl<A, B> Buffer for EitherBuffer<A, B>
where
    A: Buffer,
    B: Buffer<Element = A::Element>,
{
    type Element = A::Element;

    fn capacity(&self) -> usize {
        match self {
            EitherBuffer::First(buf) => buf.capacity(),
            EitherBuffer::Second(buf) => buf.capacity(),
        }
    }

    unsafe fn read_value(&self, index: usize) -> Self::Element {
        match self {
            EitherBuffer::First(buf) => buf.read_value(index),
            EitherBuffer::Second(buf) => buf.read_value(index),
        }
    }

    unsafe fn write_value(&mut self, index: usize, value: Self::Element) {
        match self {
            EitherBuffer::First(buf) => buf.write_value(index, value),
            EitherBuffer::Second(buf) => buf.write_value(index, value),
        }
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        match self {
            EitherBuffer::First(buf) => buf.manually_drop(index),
            EitherBuffer::Second(buf) => buf.manually_drop(index),
        }
    }

    unsafe fn manually_drop_range<R: RangeBounds<usize>>(&mut self, values_range: R) {
        match self {
            EitherBuffer::First(buf) => buf.manually_drop_range(values_range),
            EitherBuffer::Second(buf) => buf.manually_drop_range(values_range),
        }
    }

    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        match self {
            EitherBuffer::First(buf) => buf.try_grow(target),
            EitherBuffer::Second(buf) => buf.try_grow(target),
        }
    }

    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError> {
        match self {
            EitherBuffer::First(buf) => buf.try_shrink(target),
            EitherBuffer::Second(buf) => buf.try_shrink(target),
        }
    }
}

impl<A, B> PtrBuffer for EitherBuffer<A, B>
where
    A: PtrBuffer,
    B: Buffer<Element = A::Element>
        + PtrBuffer<ConstantPointer = A::ConstantPointer, MutablePointer = A::MutablePointer>,
{
    type ConstantPointer = A::ConstantPointer;
    type MutablePointer = A::MutablePointer;

    unsafe fn ptr(&self, index: usize) -> Self::ConstantPointer {
        match self {
            EitherBuffer::First(buf) => buf.ptr(index),
            EitherBuffer::Second(buf) => buf.ptr(index),
        }
    }

    unsafe fn mut_ptr(&mut self, index: usize) -> Self::MutablePointer {
        match self {
            EitherBuffer::First(buf) => buf.mut_ptr(index),
            EitherBuffer::Second(buf) => buf.mut_ptr(index),
        }
    }
}

impl<A, B> RefBuffer for EitherBuffer<A, B>
where
    A: RefBuffer,
    B: Buffer<Element = A::Element>,

    for<'a> B: RefBuffer<
            ConstantReference<'a> = A::ConstantReference<'a>,
            MutableReference<'a> = A::MutableReference<'a>,
        > + 'a,
{
    type ConstantReference<'a> = A::ConstantReference<'a>
    where
        Self: 'a;

    type MutableReference<'a> = A::MutableReference<'a>
    where
        Self: 'a;

    unsafe fn index(&self, index: usize) -> Self::ConstantReference<'_> {
        match self {
            EitherBuffer::First(buf) => buf.index(index),
            EitherBuffer::Second(buf) => buf.index(index),
        }
    }

    unsafe fn mut_index(&mut self, index: usize) -> Self::MutableReference<'_> {
        match self {
            EitherBuffer::First(buf) => buf.mut_index(index),
            EitherBuffer::Second(buf) => buf.mut_index(index),
        }
    }
}

impl<A, B> ContinuousMemoryBuffer for EitherBuffer<A, B>
where
    A: ContinuousMemoryBuffer,
    B: Buffer<Element = A::Element> + ContinuousMemoryBuffer,
{
}
