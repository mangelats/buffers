use core::slice;
use std::mem::MaybeUninit;

use crate::interface::{
    contiguous_memory::ContiguousMemoryBuffer, copy_value::CopyValueBuffer, ptrs::PtrBuffer,
    refs::RefBuffer, Buffer, ResizeError,
};

/// Buffer which works on top of a mutable slice of maybe-uninit values.
///
/// This is useful if you already have such slice and would like to modify it
/// using this interface or a regular container (for example to populate it).
///
/// It's worth noting that, like any other buffer, it doesn't have information
/// about which values are set and which are not. This information needs to be
/// handled separately (eg. giving the current size to a vector).
#[repr(transparent)]
pub struct SliceBuffer<'a, T> {
    slice: &'a mut [MaybeUninit<T>],
}

impl<'a, T> SliceBuffer<'a, T> {
    /// Makes a `SliceBuffer` from its underlying maybe-uninitialized mutable
    /// slice.
    ///
    /// Note: To use it as a buffer, the caller must know the state its in.
    pub fn from_slice(slice: &'a mut [MaybeUninit<T>]) -> Self {
        Self { slice }
    }

    /// Utility constructor that takes a `SliceBuffer` from an slice parts
    /// (pointer and length).
    ///
    /// # Safety
    /// The parts must make a valid slice. See
    /// [`core::slice::from_raw_parts_mut`] for further information.
    pub unsafe fn from_raw_slice_parts(ptr: *mut T, len: usize) -> Self {
        let data = ptr as *mut MaybeUninit<T>;
        // SAFETY: This function explicitly requires the caler to fulfill the
        // `slice::from_raw_parts_mut` safety requirements.
        let slice = unsafe { slice::from_raw_parts_mut(data, len) };
        Self { slice }
    }

    /// Internal utility that reads `index`. Used both for copying and for
    /// extracting the value.
    ///
    /// # Safety
    ///   * `index` must be less than `capacity`.
    ///   * The `index` position must be filled.
    unsafe fn read(&self, index: usize) -> T {
        // SAFETY: the Buffer interface requires the position to exist which
        // means it must have been writen into before.
        unsafe { self.slice[index].assume_init_read() }
    }
}

impl<'a, T> Buffer for SliceBuffer<'a, T> {
    type Element = T;

    fn capacity(&self) -> usize {
        self.slice.len()
    }

    unsafe fn take(&mut self, index: usize) -> Self::Element {
        // SAFETY: same requirements
        unsafe { self.read(index) }
    }

    unsafe fn put(&mut self, index: usize, value: Self::Element) {
        self.slice[index] = MaybeUninit::new(value);
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        // SAFETY: the Buffer interface requires the position to exist which
        // means it must have been writen into before.
        unsafe { self.slice[index].assume_init_drop() }
    }

    unsafe fn try_grow(&mut self, _target: usize) -> Result<(), ResizeError> {
        Err(ResizeError::UnsupportedOperation)
    }

    unsafe fn try_shrink(&mut self, _target: usize) -> Result<(), ResizeError> {
        Err(ResizeError::UnsupportedOperation)
    }
}

impl<'a, T: Copy> CopyValueBuffer for SliceBuffer<'a, T> {
    unsafe fn copy(&self, index: usize) -> T {
        // SAFETY: it has the same requirements
        unsafe { self.read(index) }
    }
}

impl<'a, T> PtrBuffer for SliceBuffer<'a, T> {
    type ConstantPointer = *const T;
    type MutablePointer = *mut T;

    unsafe fn ptr(&self, index: usize) -> Self::ConstantPointer {
        self.slice[index].as_ptr()
    }

    unsafe fn mut_ptr(&mut self, index: usize) -> Self::MutablePointer {
        self.slice[index].as_mut_ptr()
    }
}

impl<'a, T> RefBuffer for SliceBuffer<'a, T> {
    type ConstantReference<'b> = &'b T
    where
        Self: 'b;
    type MutableReference<'b> = &'b mut T
    where
        Self: 'b;

    unsafe fn index<'x: 'y, 'y>(&'x self, index: usize) -> &'y T {
        // SAFETY: `index` is unsafe with requirements that ensures that
        // [`PtrBuffer::ptr`] can be used.
        let ptr = unsafe { self.ptr(index) };
        // SAFETY: [`PtrBuffer::ptr`] ensures that the pointer can be
        // derefferenced.
        unsafe { &*ptr }
    }

    unsafe fn mut_index<'x: 'y, 'y>(&'x mut self, index: usize) -> &'y mut T {
        // SAFETY: `mut_index` is unsafe with requirements that ensures that
        // [`PtrBuffer::mut_ptr`] can be used.
        let ptr = unsafe { self.mut_ptr(index) };
        // SAFETY: [`PtrBuffer::mut_ptr`] ensures that the pointer can be
        // derefferenced.
        unsafe { &mut *ptr }
    }
}

impl<'a, T> ContiguousMemoryBuffer for SliceBuffer<'a, T> {}

#[cfg(test)]
mod tests {
    use std::mem::MaybeUninit;

    use crate::interface::Buffer;

    use super::SliceBuffer;

    #[test]
    fn can_be_constructed_from_slice() {
        let mut array = MaybeUninit::<u32>::uninit_array::<10>();
        let slice = &mut array[..];

        let mut buffer = SliceBuffer::from_slice(slice);

        const VALUE: u32 = 123;
        unsafe { buffer.put(0, VALUE) };
        let result = unsafe { buffer.take(0) };
        assert_eq!(result, VALUE);
    }

    #[test]
    fn can_be_constructed_from_slice_parts() {
        const SIZE: usize = 10;
        let mut array = MaybeUninit::<u32>::uninit_array::<10>();
        let ptr = array.as_mut_ptr() as *mut u32;

        let mut buffer = unsafe { SliceBuffer::from_raw_slice_parts(ptr, SIZE) };

        const VALUE: u32 = 456;
        unsafe { buffer.put(0, VALUE) };
        let result = unsafe { buffer.take(0) };
        assert_eq!(result, VALUE);
    }
}
