use std::{mem::MaybeUninit, ops::RangeBounds};

use crate::interface::{copy_value::CopyValueBuffer, Buffer, ResizeError};

/// Buffer that given a fixed-size array, it makes a buffer the underlying
/// layout of which is an array of buffers of the array's element type. This is
/// similar to a SoA but all the buffers are the same type (`B`) and have the
/// same element type.
///
/// ```rust
/// # use buffers::interface::Buffer;
/// # use buffers::base_buffers::HeapBuffer;
/// # use buffers::composites::ArrayBuffer;
/// let mut buffer: ArrayBuffer<2, HeapBuffer<u32>> = Default::default();
/// unsafe {
///     buffer.try_grow(10);
///     buffer.write_value(0, [1, 2]);
///     buffer.write_value(1, [4, 5]);
/// }
///
/// assert_eq!(unsafe { buffer.read_value(0) }, [1, 2]);
/// assert_eq!(unsafe { buffer.read_value(1) }, [4, 5]);
/// ```
#[repr(transparent)]
pub struct ArrayBuffer<const SIZE: usize, B>
where
    B: Buffer,
{
    buffers: [B; SIZE],
}

impl<const SIZE: usize, B> ArrayBuffer<SIZE, B>
where
    B: Buffer,
{
    /// Make a new [`ArrayBuffer<SIZE, B>`] given the underlying array of
    /// buffers.
    pub fn from(buffers: [B; SIZE]) -> Self {
        Self { buffers }
    }

    /// Helper function to iterate over all inner buffers
    fn buffer_iter(&self) -> impl Iterator<Item = &B> {
        self.buffers.as_slice().iter()
    }

    /// Helper function to iterate over all inner buffers
    fn buffer_iter_mut(&mut self) -> impl Iterator<Item = &mut B> {
        self.buffers.as_mut_slice().iter_mut()
    }
}

impl<const SIZE: usize, B> ArrayBuffer<SIZE, B>
where
    B: Buffer,
    B: Default,
{
    /// Creates a new [`ArrayBuffer`] by default constructing the underlying
    /// buffers.
    fn new() -> Self {
        Self {
            buffers: default_array(),
        }
    }
}
impl<const SIZE: usize, B> Default for ArrayBuffer<SIZE, B>
where
    B: Buffer,
    B: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const SIZE: usize, B> Buffer for ArrayBuffer<SIZE, B>
where
    B: Buffer,
{
    type Element = [B::Element; SIZE];

    fn capacity(&self) -> usize {
        self.buffers.iter().map(B::capacity).min().unwrap_or(0)
    }

    unsafe fn read_value(&mut self, index: usize) -> Self::Element {
        let mut result = MaybeUninit::<B::Element>::uninit_array::<SIZE>();
        for (i, buffer) in self.buffer_iter_mut().enumerate() {
            let ptr = result[i].as_mut_ptr();

            // SAFETY: if `index` is a valid and filled position to this buffer,
            // it's also valid and filled for all the underlying ones.
            let val = unsafe { buffer.read_value(index) };
            // SAFETY: `ptr` is part of a local array, thus a valid location
            // (and without a value).
            unsafe { ptr.write(val) };
        }

        // SAFETY: the loop filled the entire array, thus it's initialized.
        unsafe { MaybeUninit::array_assume_init(result) }
    }

    unsafe fn write_value(&mut self, index: usize, value: Self::Element) {
        for (buffer, v) in self.buffer_iter_mut().zip(value) {
            // SAFETY: if `index` is a valid and empty position to this buffer,
            // it's also valid and empty for all the underlying ones.
            unsafe { buffer.write_value(index, v) }
        }
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        for buffer in self.buffer_iter_mut() {
            // SAFETY: if `index` is a valid and filled position to this buffer,
            // it's also valid and filled for all the underlying ones.
            unsafe { buffer.manually_drop(index) }
        }
    }

    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        for buffer in self.buffer_iter_mut() {
            if buffer.capacity() < target {
                // SAFETY: Conditional guards precondition.
                match unsafe { buffer.try_grow(target) } {
                    Ok(_) => {}
                    Err(ResizeError::UnsupportedOperation) => {}
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(())
    }

    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), crate::interface::ResizeError> {
        for buffer in self.buffer_iter_mut() {
            // SAFETY: `self.capacity()` <= `inner_buffer.capacity()`. Thus
            // `target` < `inner_buffer.capacity()` for all inner buffers.
            match unsafe { buffer.try_shrink(target) } {
                Ok(_) => {}
                Err(ResizeError::UnsupportedOperation) => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    unsafe fn manually_drop_range<R: RangeBounds<usize> + Clone>(&mut self, values_range: R) {
        for buffer in self.buffer_iter_mut() {
            let range = values_range.clone();
            // SAFETY: Forwarding call to inner buffers.
            unsafe { buffer.manually_drop_range(range) };
        }
    }

    unsafe fn shift_right<R: RangeBounds<usize> + Clone>(&mut self, to_move: R, positions: usize) {
        for buffer in self.buffer_iter_mut() {
            let range = to_move.clone();
            // SAFETY: Forwarding call to inner buffers.
            unsafe { buffer.shift_right(range, positions) };
        }
    }

    unsafe fn shift_left<R: RangeBounds<usize> + Clone>(&mut self, to_move: R, positions: usize) {
        for buffer in self.buffer_iter_mut() {
            let range = to_move.clone();
            // SAFETY: Forwarding call to inner buffers.
            unsafe { buffer.shift_left(range, positions) };
        }
    }
}

impl<const SIZE: usize, B> CopyValueBuffer for ArrayBuffer<SIZE, B>
where
    B: CopyValueBuffer,
    B::Element: Copy,
{
    unsafe fn copy_value(&self, index: usize) -> Self::Element {
        let mut result = MaybeUninit::<B::Element>::uninit_array::<SIZE>();
        for (i, buffer) in self.buffer_iter().enumerate() {
            let ptr = result[i].as_mut_ptr();

            // SAFETY: if `index` is a valid and filled position to this buffer,
            // it's also valid and filled for all the underlying ones.
            let val = unsafe { buffer.copy_value(index) };
            // SAFETY: `ptr` is part of a local array, thus a valid location
            // (and without a value).
            unsafe { ptr.write(val) };
        }

        // SAFETY: the loop filled the entire array, thus it's initialized.
        unsafe { MaybeUninit::array_assume_init(result) }
    }
}

/// Helper function. It cretes a default fixed-size array for any T which is
/// [`Default`].
fn default_array<T: Default, const N: usize>() -> [T; N] {
    let mut result = MaybeUninit::<T>::uninit_array::<N>();
    for position in result.iter_mut() {
        // SAFETY: All positions are empty before the loop. The loop visits them
        // only once. This the moving on each value is valid.
        unsafe { position.as_mut_ptr().write(Default::default()) }
    }

    // SAFETY: All values have been set on the previous loop
    unsafe { MaybeUninit::array_assume_init(result) }
}
