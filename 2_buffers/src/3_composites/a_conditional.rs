use std::{marker::PhantomData, ops::RangeBounds};

use crate::interface::{
    continuous_memory::ContinuousMemoryBuffer, ptrs::PtrBuffer, refs::RefBuffer,
    resize_error::ResizeError, Buffer,
};

/// Trait used to choose between buffer A or buffer B
pub trait Selector {
    const SELECT_A: bool;
}

/// Utility composite buffer that allows to use one buffer or another defined at compilation time.
///
/// Note that this uses both buffers but only uses one. This may be able to change
/// with generic const expressions.
pub struct ConditionalBuffer<T, A, B, S>
where
    A: Buffer<Element = T>,
    B: Buffer<Element = T>,
    S: Selector,
{
    a: A,
    b: B,
    _m: PhantomData<(T, S)>,
}

impl<T, A, B, S> ConditionalBuffer<T, A, B, S>
where
    A: Buffer<Element = T>,
    B: Buffer<Element = T>,
    S: Selector,
{
    pub fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            _m: PhantomData,
        }
    }
}

impl<T, A, B, S> Default for ConditionalBuffer<T, A, B, S>
where
    A: Buffer<Element = T> + Default,
    B: Buffer<Element = T> + Default,
    S: Selector,
{
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<T, A, B, S> Buffer for ConditionalBuffer<T, A, B, S>
where
    A: Buffer<Element = T>,
    B: Buffer<Element = T>,
    S: Selector,
{
    type Element = T;
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

    unsafe fn manually_drop_range<R: RangeBounds<usize>>(&mut self, values_range: R) {
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

impl<T, A, B, S> PtrBuffer for ConditionalBuffer<T, A, B, S>
where
    A: Buffer<Element = T> + PtrBuffer,
    B: Buffer<Element = T>
        + PtrBuffer<ConstantPointer = A::ConstantPointer, MutablePointer = A::MutablePointer>,
    S: Selector,
{
    type ConstantPointer = A::ConstantPointer;
    type MutablePointer = A::MutablePointer;

    unsafe fn ptr(&self, index: usize) -> Self::ConstantPointer {
        if S::SELECT_A {
            self.a.ptr(index)
        } else {
            self.b.ptr(index)
        }
    }

    unsafe fn mut_ptr(&mut self, index: usize) -> Self::MutablePointer {
        if S::SELECT_A {
            self.a.mut_ptr(index)
        } else {
            self.b.mut_ptr(index)
        }
    }
}

impl<T, A, B, S> RefBuffer for ConditionalBuffer<T, A, B, S>
where
    A: Buffer<Element = T> + RefBuffer,
    B: Buffer<Element = T>,
    for<'a> B: RefBuffer<
            ConstantReference<'a> = A::ConstantReference<'a>,
            MutableReference<'a> = A::MutableReference<'a>,
        > + 'a,
    S: Selector,
{
    type ConstantReference<'a> = A::ConstantReference<'a>
    where
        Self: 'a;

    type MutableReference<'a> = A::MutableReference<'a>
    where
        Self: 'a;

    unsafe fn index(&self, index: usize) -> Self::ConstantReference<'_> {
        if S::SELECT_A {
            self.a.index(index)
        } else {
            self.b.index(index)
        }
    }

    unsafe fn mut_index(&mut self, index: usize) -> Self::MutableReference<'_> {
        if S::SELECT_A {
            self.a.mut_index(index)
        } else {
            self.b.mut_index(index)
        }
    }
}

impl<T, A, B, S> ContinuousMemoryBuffer for ConditionalBuffer<T, A, B, S>
where
    A: Buffer<Element = T> + ContinuousMemoryBuffer,
    B: Buffer<Element = T> + ContinuousMemoryBuffer,
    S: Selector,
{
}
