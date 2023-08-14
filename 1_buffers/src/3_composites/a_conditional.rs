use std::{marker::PhantomData, mem::MaybeUninit, ops::RangeBounds};

use crate::interface::{
    contiguous_memory::ContiguousMemoryBuffer, ptrs::PtrBuffer, refs::RefBuffer,
    resize_error::ResizeError, Buffer,
};

/// Trait used to choose between buffer A or buffer B.
///
/// This is necessary due to restrictions in const expressions in generic
/// arguments. But because is technically used publicly, it needs to be public.
pub trait Selector {
    const SELECT_A: bool;
}

/// Utility composite buffer that allows to use one buffer or another defined at
/// compilation time.
///
/// This uses both buffers but only uses one. This may be able to change with
/// generic const expressions.
pub struct ConditionalBuffer<A, B, S>
where
    A: Buffer,
    B: Buffer<Element = A::Element>,
    S: Selector,
{
    a: MaybeUninit<A>,
    b: MaybeUninit<B>,
    _m: PhantomData<S>,
}

impl<A, B, S> ConditionalBuffer<A, B, S>
where
    A: Buffer,
    B: Buffer<Element = A::Element>,
    S: Selector,
{
    /// Creates the buffer by using the first (`A`) option
    pub fn with_first(first: A) -> Self {
        debug_assert!(
            S::SELECT_A,
            "Should select A to create ConditionalBuffer with A"
        );
        Self {
            a: MaybeUninit::new(first),
            b: MaybeUninit::uninit(),
            _m: PhantomData,
        }
    }
    /// Creates the buffer by using the second (`B`) option
    pub fn with_second(second: B) -> Self {
        debug_assert!(
            !S::SELECT_A,
            "Should not select A to create ConditionalBuffer with B"
        );
        Self {
            a: MaybeUninit::uninit(),
            b: MaybeUninit::new(second),
            _m: PhantomData,
        }
    }
}

impl<A, B, S> Default for ConditionalBuffer<A, B, S>
where
    A: Buffer + Default,
    B: Buffer<Element = A::Element> + Default,
    S: Selector,
{
    fn default() -> Self {
        if S::SELECT_A {
            Self::with_first(Default::default())
        } else {
            Self::with_second(Default::default())
        }
    }
}

impl<A, B, S> Buffer for ConditionalBuffer<A, B, S>
where
    A: Buffer,
    B: Buffer<Element = A::Element>,
    S: Selector,
{
    type Element = A::Element;
    fn capacity(&self) -> usize {
        if S::SELECT_A {
            unsafe { self.a.assume_init_ref() }.capacity()
        } else {
            unsafe { self.b.assume_init_ref() }.capacity()
        }
    }

    unsafe fn read_value(&self, index: usize) -> Self::Element {
        if S::SELECT_A {
            unsafe { self.a.assume_init_ref() }.read_value(index)
        } else {
            unsafe { self.b.assume_init_ref() }.read_value(index)
        }
    }

    unsafe fn write_value(&mut self, index: usize, value: Self::Element) {
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

impl<A, B, S> PtrBuffer for ConditionalBuffer<A, B, S>
where
    A: PtrBuffer,
    B: Buffer<Element = A::Element>
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

impl<A, B, S> RefBuffer for ConditionalBuffer<A, B, S>
where
    A: RefBuffer,
    B: Buffer<Element = A::Element>,

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

impl<A, B, S> ContiguousMemoryBuffer for ConditionalBuffer<A, B, S>
where
    A: ContiguousMemoryBuffer,
    B: Buffer<Element = A::Element> + ContiguousMemoryBuffer,
    S: Selector,
{
}

impl<A, B, S> Drop for ConditionalBuffer<A, B, S>
where
    A: Buffer,
    B: Buffer<Element = A::Element>,
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
