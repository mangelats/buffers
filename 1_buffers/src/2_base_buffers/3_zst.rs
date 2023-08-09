use std::marker::PhantomData;

use crate::interface::{ptrs::PtrBuffer, refs::DefaultRefBuffer, Buffer};

/// Buffer optimized for zero-sized types.
///
/// Zero-sized types don't have any space, so they actually don't need allocation at all.
///
/// Note that this buffer is a zero-sized type.
pub struct ZstBuffer<T> {
    _m: PhantomData<T>,
}

impl<T> ZstBuffer<T> {
    /// Makes a new zero-sized type buffer.
    ///
    /// It has a debug assert to make sure the type is a ZST.
    pub fn new() -> Self {
        debug_assert_eq!(
            std::mem::size_of::<T>(),
            0,
            "ZstBuffer only works with zero-sized types"
        );
        Self { _m: PhantomData }
    }
}

impl<T> Buffer for ZstBuffer<T> {
    type Element = T;

    fn capacity(&self) -> usize {
        usize::MAX
    }

    unsafe fn read_value(&self, _index: usize) -> T {
        // SAFETY: This type has no size. Null should work as well as any other pointer.
        // TODO: adding an intrinsics::assume for the size of T may be worth
        std::ptr::read(std::ptr::NonNull::dangling().as_ptr())
    }

    unsafe fn write_value(&mut self, _index: usize, _value: T) {
        // Do nothing
    }

    unsafe fn manually_drop(&mut self, _index: usize) {
        // Do nothing
    }
}

impl<T> Default for ZstBuffer<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PtrBuffer for ZstBuffer<T> {
    type ConstantPointer = *const T;
    type MutablePointer = *mut T;

    unsafe fn ptr(&self, _index: usize) -> *const Self::Element {
        std::ptr::NonNull::dangling().as_ptr()
    }

    unsafe fn mut_ptr(&mut self, _index: usize) -> *mut Self::Element {
        std::ptr::NonNull::dangling().as_ptr()
    }
}
impl<T> DefaultRefBuffer for ZstBuffer<T> {}
