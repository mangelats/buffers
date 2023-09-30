// All unsafe are is just forwaring to underlying buffers.
#![allow(clippy::undocumented_unsafe_blocks)]

use std::ops::RangeBounds;

use crate::interface::{
    contiguous_memory::ContiguousMemoryBuffer, copy_value::CopyValueBuffer, ptrs::PtrBuffer,
    refs::RefBuffer, resize_error::ResizeError, Buffer,
};

/// Utility buffer that may contain one of two buffers.
///
/// It's a Buffer itself, forwarding the requests to the currently selected.
pub enum EitherBuffer<A, B>
where
    A: Buffer,
    B: Buffer<Element = A::Element>,
{
    /// First option (`A` buffer)
    First(A),
    /// Second option (`B` buffer)
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

    unsafe fn read_value(&mut self, index: usize) -> Self::Element {
        match self {
            EitherBuffer::First(buf) => unsafe { buf.read_value(index) },
            EitherBuffer::Second(buf) => unsafe { buf.read_value(index) },
        }
    }

    unsafe fn write_value(&mut self, index: usize, value: Self::Element) {
        match self {
            EitherBuffer::First(buf) => unsafe { buf.write_value(index, value) },
            EitherBuffer::Second(buf) => unsafe { buf.write_value(index, value) },
        }
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        match self {
            EitherBuffer::First(buf) => unsafe { buf.manually_drop(index) },
            EitherBuffer::Second(buf) => unsafe { buf.manually_drop(index) },
        }
    }

    unsafe fn manually_drop_range<R: RangeBounds<usize> + Clone>(&mut self, values_range: R) {
        match self {
            EitherBuffer::First(buf) => unsafe { buf.manually_drop_range(values_range) },
            EitherBuffer::Second(buf) => unsafe { buf.manually_drop_range(values_range) },
        }
    }

    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        match self {
            EitherBuffer::First(buf) => unsafe { buf.try_grow(target) },
            EitherBuffer::Second(buf) => unsafe { buf.try_grow(target) },
        }
    }

    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError> {
        match self {
            EitherBuffer::First(buf) => unsafe { buf.try_shrink(target) },
            EitherBuffer::Second(buf) => unsafe { buf.try_shrink(target) },
        }
    }
}

impl<A, B> CopyValueBuffer for EitherBuffer<A, B>
where
    A: Buffer + CopyValueBuffer,
    A::Element: Copy,
    B: Buffer<Element = A::Element> + CopyValueBuffer,
{
    unsafe fn copy(&self, index: usize) -> Self::Element {
        match self {
            EitherBuffer::First(buf) => unsafe { buf.copy(index) },
            EitherBuffer::Second(buf) => unsafe { buf.copy(index) },
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
            EitherBuffer::First(buf) => unsafe { buf.ptr(index) },
            EitherBuffer::Second(buf) => unsafe { buf.ptr(index) },
        }
    }

    unsafe fn mut_ptr(&mut self, index: usize) -> Self::MutablePointer {
        match self {
            EitherBuffer::First(buf) => unsafe { buf.mut_ptr(index) },
            EitherBuffer::Second(buf) => unsafe { buf.mut_ptr(index) },
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

    unsafe fn index<'a: 'b, 'b>(&'a self, index: usize) -> Self::ConstantReference<'b> {
        match self {
            EitherBuffer::First(buf) => unsafe { buf.index(index) },
            EitherBuffer::Second(buf) => unsafe { buf.index(index) },
        }
    }

    unsafe fn mut_index<'a: 'b, 'b>(&'a mut self, index: usize) -> Self::MutableReference<'b> {
        match self {
            EitherBuffer::First(buf) => unsafe { buf.mut_index(index) },
            EitherBuffer::Second(buf) => unsafe { buf.mut_index(index) },
        }
    }
}

impl<A, B> ContiguousMemoryBuffer for EitherBuffer<A, B>
where
    A: ContiguousMemoryBuffer,
    B: Buffer<Element = A::Element> + ContiguousMemoryBuffer,
{
}
