use std::marker::PhantomData;

use crate::interface::Buffer;

pub struct ZstBuffer<T> {
    _m: PhantomData<T>,
}

impl<T> ZstBuffer<T> {
    fn new() -> Self {
        debug_assert_eq!(
            std::mem::size_of::<T>(),
            0,
            "ZstBuffer only works with zero-sized types"
        );
        Self { _m: PhantomData }
    }
}

impl<T> Buffer<T> for ZstBuffer<T> {
    fn capacity(&self) -> usize {
        usize::MAX
    }

    unsafe fn read_value(&self, index: usize) -> T {
        // SAFETY: This type has no size. Null should work as well as any other pointer.
        // TODO: adding an intrinsics::assume for the size of T may be worth
        std::ptr::read(std::ptr::NonNull::dangling().as_ptr())
    }

    unsafe fn write_value(&mut self, index: usize, value: T) {}

    unsafe fn manually_drop(&mut self, index: usize) {}
}
