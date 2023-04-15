use std::{marker::PhantomData, ops::Range};

use crate::interface::{resize_error::ResizeError, Buffer};

pub struct ConditionalBuffer<T, A: Buffer<T>, B: Buffer<T>, const SELECT_A: bool> {
    a: A,
    b: B,
    _m: PhantomData<T>,
}

impl<T, A: Buffer<T>, B: Buffer<T>, const SELECT_A: bool> Buffer<T>
    for ConditionalBuffer<T, A, B, SELECT_A>
{
    fn capacity(&self) -> usize {
        if SELECT_A {
            self.a.capacity()
        } else {
            self.b.capacity()
        }
    }

    unsafe fn read_value(&self, index: usize) -> T {
        if SELECT_A {
            self.a.read_value(index)
        } else {
            self.b.read_value(index)
        }
    }

    unsafe fn write_value(&mut self, index: usize, value: T) {
        if SELECT_A {
            self.a.write_value(index, value)
        } else {
            self.b.write_value(index, value)
        }
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        if SELECT_A {
            self.a.manually_drop(index)
        } else {
            self.b.manually_drop(index)
        }
    }

    unsafe fn manually_drop_range(&mut self, values_range: Range<usize>) {
        if SELECT_A {
            self.a.manually_drop_range(values_range)
        } else {
            self.b.manually_drop_range(values_range)
        }
    }
    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        if SELECT_A {
            self.a.try_grow(target)
        } else {
            self.b.try_grow(target)
        }
    }

    unsafe fn try_shrink(&mut self, _target: usize) -> Result<(), ResizeError> {
        Err(ResizeError::UnsupportedOperation)
    }
}
