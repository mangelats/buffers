use std::{marker::PhantomData, ops::RangeBounds};

use crate::{
    interface::{
        continuous_memory::ContinuousMemoryBuffer, ptrs::PtrBuffer, refs::RefBuffer,
        resize_error::ResizeError, Buffer,
    },
    never::Never,
};

/// Utility buffer that may contain one of two buffers.
///
/// It's a Buffer itself, forwarding the requests to the currently selected.
pub enum EitherBuffer<T, A, B>
where
    A: Buffer<Element = T>,
    B: Buffer<Element = T>,
{
    /// First option (A buffer)
    First(A),
    /// Second option (B buffer)
    Second(B),

    /// Internal option that never can be selected which holds PhantomData to T
    _InternalMarker(Never, PhantomData<T>),
}

impl<T, A, B> Default for EitherBuffer<T, A, B>
where
    A: Buffer<Element = T> + Default,
    B: Buffer<Element = T>,
{
    fn default() -> Self {
        Self::First(Default::default())
    }
}

impl<T, A, B> Buffer for EitherBuffer<T, A, B>
where
    A: Buffer<Element = T>,
    B: Buffer<Element = T>,
{
    type Element = T;

    fn capacity(&self) -> usize {
        match self {
            EitherBuffer::First(buf) => buf.capacity(),
            EitherBuffer::Second(buf) => buf.capacity(),
            EitherBuffer::_InternalMarker(_, _) => unreachable!(),
        }
    }

    unsafe fn read_value(&self, index: usize) -> T {
        match self {
            EitherBuffer::First(buf) => buf.read_value(index),
            EitherBuffer::Second(buf) => buf.read_value(index),
            EitherBuffer::_InternalMarker(_, _) => unreachable!(),
        }
    }

    unsafe fn write_value(&mut self, index: usize, value: T) {
        match self {
            EitherBuffer::First(buf) => buf.write_value(index, value),
            EitherBuffer::Second(buf) => buf.write_value(index, value),
            EitherBuffer::_InternalMarker(_, _) => unreachable!(),
        }
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        match self {
            EitherBuffer::First(buf) => buf.manually_drop(index),
            EitherBuffer::Second(buf) => buf.manually_drop(index),
            EitherBuffer::_InternalMarker(_, _) => unreachable!(),
        }
    }

    unsafe fn manually_drop_range<R: RangeBounds<usize>>(&mut self, values_range: R) {
        match self {
            EitherBuffer::First(buf) => buf.manually_drop_range(values_range),
            EitherBuffer::Second(buf) => buf.manually_drop_range(values_range),
            EitherBuffer::_InternalMarker(_, _) => unreachable!(),
        }
    }

    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        match self {
            EitherBuffer::First(buf) => buf.try_grow(target),
            EitherBuffer::Second(buf) => buf.try_grow(target),
            EitherBuffer::_InternalMarker(_, _) => unreachable!(),
        }
    }

    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError> {
        match self {
            EitherBuffer::First(buf) => buf.try_shrink(target),
            EitherBuffer::Second(buf) => buf.try_shrink(target),
            EitherBuffer::_InternalMarker(_, _) => unreachable!(),
        }
    }
}

impl<T, A, B> PtrBuffer for EitherBuffer<T, A, B>
where
    A: Buffer<Element = T> + PtrBuffer,
    B: Buffer<Element = T>
        + PtrBuffer<ConstantPointer = A::ConstantPointer, MutablePointer = A::MutablePointer>,
{
    type ConstantPointer = A::ConstantPointer;
    type MutablePointer = A::MutablePointer;

    unsafe fn ptr(&self, index: usize) -> Self::ConstantPointer {
        match self {
            EitherBuffer::First(buf) => buf.ptr(index),
            EitherBuffer::Second(buf) => buf.ptr(index),
            EitherBuffer::_InternalMarker(_, _) => unreachable!(),
        }
    }

    unsafe fn mut_ptr(&mut self, index: usize) -> Self::MutablePointer {
        match self {
            EitherBuffer::First(buf) => buf.mut_ptr(index),
            EitherBuffer::Second(buf) => buf.mut_ptr(index),
            EitherBuffer::_InternalMarker(_, _) => unreachable!(),
        }
    }
}

impl<T, A, B> RefBuffer for EitherBuffer<T, A, B>
where
    A: Buffer<Element = T> + RefBuffer,
    B: Buffer<Element = T>,
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
            EitherBuffer::_InternalMarker(_, _) => unreachable!(),
        }
    }

    unsafe fn mut_index(&mut self, index: usize) -> Self::MutableReference<'_> {
        match self {
            EitherBuffer::First(buf) => buf.mut_index(index),
            EitherBuffer::Second(buf) => buf.mut_index(index),
            EitherBuffer::_InternalMarker(_, _) => unreachable!(),
        }
    }
}

impl<T, A, B> ContinuousMemoryBuffer for EitherBuffer<T, A, B>
where
    A: Buffer<Element = T> + ContinuousMemoryBuffer,
    B: Buffer<Element = T> + ContinuousMemoryBuffer,
{
}
