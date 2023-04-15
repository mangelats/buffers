use std::marker::PhantomData;

use crate::interface::Buffer;

pub enum EitherBuffer<T, A: Buffer<T>, B: Buffer<T>> {
    /// First option (A buffer)
    First(A),
    /// Second option (B buffer)
    Second(B),

    /// Internal option that never can be selected which holds PhantomData to T
    _InternalMarker(Never, PhantomData<T>),
}

impl<T, A: Buffer<T>, B: Buffer<T>> Buffer<T> for EitherBuffer<T, A, B> {
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
        todo!()
    }
}

/// A type that can never exist.
///
/// Equivalent to the never type (!) which is experimental
/// (see issue #35121 <https://github.com/rust-lang/rust/issues/35121>).
pub enum Never {}
