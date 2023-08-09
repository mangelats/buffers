use std::{marker::PhantomData, mem::MaybeUninit, ops::RangeBounds};

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
    a: MaybeUninit<A>,
    b: MaybeUninit<B>,
    _m: PhantomData<(T, S)>,
}

impl<T, A, B, S> ConditionalBuffer<T, A, B, S>
where
    A: Buffer<Element = T>,
    B: Buffer<Element = T>,
    S: Selector,
{
    pub fn with_a(a: A) -> Self {
        debug_assert!(
            S::SELECT_A,
            "Should select A to create ConditionalBuffer with A"
        );
        Self {
            a: MaybeUninit::new(a),
            b: MaybeUninit::uninit(),
            _m: PhantomData,
        }
    }
    pub fn with_b(b: B) -> Self {
        debug_assert!(
            !S::SELECT_A,
            "Should not select A to create ConditionalBuffer with B"
        );
        Self {
            a: MaybeUninit::uninit(),
            b: MaybeUninit::new(b),
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
        if S::SELECT_A {
            Self::with_a(Default::default())
        } else {
            Self::with_b(Default::default())
        }
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
            unsafe { self.a.assume_init_ref() }.capacity()
        } else {
            unsafe { self.b.assume_init_ref() }.capacity()
        }
    }

    unsafe fn read_value(&self, index: usize) -> T {
        if S::SELECT_A {
            unsafe { self.a.assume_init_ref() }.read_value(index)
        } else {
            unsafe { self.b.assume_init_ref() }.read_value(index)
        }
    }

    unsafe fn write_value(&mut self, index: usize, value: T) {
        if S::SELECT_A {
            unsafe { self.a.assume_init_mut() }.write_value(index, value)
        } else {
            unsafe { self.b.assume_init_mut() }.write_value(index, value)
        }
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        if S::SELECT_A {
            unsafe { self.a.assume_init_mut() }.manually_drop(index)
        } else {
            unsafe { self.b.assume_init_mut() }.manually_drop(index)
        }
    }

    unsafe fn manually_drop_range<R: RangeBounds<usize>>(&mut self, values_range: R) {
        if S::SELECT_A {
            unsafe { self.a.assume_init_mut() }.manually_drop_range(values_range)
        } else {
            unsafe { self.b.assume_init_mut() }.manually_drop_range(values_range)
        }
    }
    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        if S::SELECT_A {
            unsafe { self.a.assume_init_mut() }.try_grow(target)
        } else {
            unsafe { self.b.assume_init_mut() }.try_grow(target)
        }
    }

    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError> {
        if S::SELECT_A {
            unsafe { self.a.assume_init_mut() }.try_shrink(target)
        } else {
            unsafe { self.b.assume_init_mut() }.try_shrink(target)
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
            unsafe { self.a.assume_init_ref() }.ptr(index)
        } else {
            unsafe { self.b.assume_init_ref() }.ptr(index)
        }
    }

    unsafe fn mut_ptr(&mut self, index: usize) -> Self::MutablePointer {
        if S::SELECT_A {
            unsafe { self.a.assume_init_mut() }.mut_ptr(index)
        } else {
            unsafe { self.b.assume_init_mut() }.mut_ptr(index)
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
            unsafe { self.a.assume_init_ref() }.index(index)
        } else {
            unsafe { self.b.assume_init_ref() }.index(index)
        }
    }

    unsafe fn mut_index(&mut self, index: usize) -> Self::MutableReference<'_> {
        if S::SELECT_A {
            unsafe { self.a.assume_init_mut() }.mut_index(index)
        } else {
            unsafe { self.b.assume_init_mut() }.mut_index(index)
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

impl<T, A, B, S> Drop for ConditionalBuffer<T, A, B, S>
where
    A: Buffer<Element = T>,
    B: Buffer<Element = T>,
    S: Selector,
{
    fn drop(&mut self) {
        if S::SELECT_A {
            unsafe { self.a.assume_init_drop() }
        } else {
            unsafe { self.b.assume_init_drop() }
        }
    }
}
