use std::{marker::PhantomData, ops::Range};

use crate::interface::{resize_error::ResizeError, Buffer};

pub trait Selector {
    const SELECT_A: bool;
}

/// Utility composite buffer that allows to use one buffer or another defined at compilation time.
///
/// Note that this uses both buffers but only uses one. This may be able to change
/// with generic const expressions.
pub struct ConditionalBuffer<T, A: Buffer<T>, B: Buffer<T>, S: Selector> {
    a: A,
    b: B,
    _m: PhantomData<(T, S)>,
}

impl<T, A: Buffer<T>, B: Buffer<T>, S: Selector> ConditionalBuffer<T, A, B, S> {
    pub fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            _m: PhantomData,
        }
    }
}

impl<T, A: Buffer<T> + Default, B: Buffer<T> + Default, S: Selector> Default
    for ConditionalBuffer<T, A, B, S>
{
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<T, A: Buffer<T>, B: Buffer<T>, S: Selector> Buffer<T> for ConditionalBuffer<T, A, B, S> {
    fn capacity(&self) -> usize {
        if S::SELECT_A {
            self.a.capacity()
        } else {
            self.b.capacity()
        }
    }

    unsafe fn read_value(&self, index: usize) -> T {
        if S::SELECT_A {
            self.a.read_value(index)
        } else {
            self.b.read_value(index)
        }
    }

    unsafe fn write_value(&mut self, index: usize, value: T) {
        if S::SELECT_A {
            self.a.write_value(index, value)
        } else {
            self.b.write_value(index, value)
        }
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        if S::SELECT_A {
            self.a.manually_drop(index)
        } else {
            self.b.manually_drop(index)
        }
    }

    unsafe fn manually_drop_range(&mut self, values_range: Range<usize>) {
        if S::SELECT_A {
            self.a.manually_drop_range(values_range)
        } else {
            self.b.manually_drop_range(values_range)
        }
    }
    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        if S::SELECT_A {
            self.a.try_grow(target)
        } else {
            self.b.try_grow(target)
        }
    }

    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError> {
        if S::SELECT_A {
            self.a.try_shrink(target)
        } else {
            self.b.try_shrink(target)
        }
    }
}
