use std::marker::PhantomData;

use crate::interface::{
    copy_value::CopyValueBuffer, ptrs::PtrBuffer, refs::RefBuffer, resize_error::ResizeError,
    Buffer,
};

/// Buffer optimized for zero-sized types.
///
/// Zero-sized types don't have any space, so they actually don't need
/// allocation at all.
///
/// Note that this buffer is also a zero-sized type.
pub struct ZstBuffer<T> {
    _m: PhantomData<T>,
}

impl<T> ZstBuffer<T> {
    /// Makes a new zero-sized type buffer.
    pub fn new() -> Self {
        // Debug assert to make sure the type is a ZST.
        debug_assert_eq!(
            std::mem::size_of::<T>(),
            0,
            "ZstBuffer only works with zero-sized types"
        );
        Self { _m: PhantomData }
    }

    /// Internal utility that reads `index`.
    ///
    /// # Safety
    ///   * `index` must be less than `capacity`.
    ///   * The `index` position must be filled.
    unsafe fn read(&self, _index: usize) -> T {
        // SAFETY: This type has no size. A dangling pointer should work as well
        // as any other pointer.
        // TODO: adding an intrinsics::assume for the size of T may increase
        // performance.
        unsafe { std::ptr::read(std::ptr::NonNull::dangling().as_ptr()) }
    }
}

impl<T> Buffer for ZstBuffer<T> {
    type Element = T;

    fn capacity(&self) -> usize {
        usize::MAX
    }

    unsafe fn take(&mut self, index: usize) -> T {
        // SAFETY: it has the same requirements
        unsafe { self.read(index) }
    }

    unsafe fn put(&mut self, _index: usize, _value: T) {
        // Do nothing
    }

    unsafe fn manually_drop(&mut self, _index: usize) {
        // Do nothing
    }

    unsafe fn try_grow(&mut self, _target: usize) -> Result<(), ResizeError> {
        Err(ResizeError::UnsupportedOperation)
    }

    unsafe fn try_shrink(&mut self, _target: usize) -> Result<(), ResizeError> {
        Err(ResizeError::UnsupportedOperation)
    }
}

impl<T: Copy> CopyValueBuffer for ZstBuffer<T> {
    unsafe fn copy(&self, index: usize) -> T {
        // SAFETY: it has the same requirements
        unsafe { self.read(index) }
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

impl<T> RefBuffer for ZstBuffer<T> {
    type ConstantReference<'a> = &'a T
    where
        Self: 'a;
    type MutableReference<'a> = &'a mut T
    where
        Self: 'a;

    unsafe fn index<'a: 'b, 'b>(&'a self, index: usize) -> &'b T {
        // SAFETY: [`RefBuffer::index`] has at least the same requirements as
        // [`PtrBuffer::ptr`].
        let ptr = unsafe { self.ptr(index) };
        // SAFETY: [`PtrBuffer::ptr`] requires that the pointer can be
        // dereferenced.
        unsafe { &*ptr }
    }

    unsafe fn mut_index<'a: 'b, 'b>(&'a mut self, index: usize) -> &'b mut T {
        // SAFETY: [`RefBuffer::mut_index`] has at least the same requirements
        // as [`PtrBuffer::mut_ptr`].
        let ptr = unsafe { self.mut_ptr(index) };
        // SAFETY: [`PtrBuffer::mut_ptr`] requires that the pointer can be
        // dereferenced.
        unsafe { &mut *ptr }
    }
}
