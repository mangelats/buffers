use std::{mem::MaybeUninit, ops::RangeBounds};

use crate::interface::{Buffer, ResizeError};

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

    fn buffer_iter(&self) -> impl Iterator<Item = &B> {
        self.buffers.as_slice().iter()
    }
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
        self.buffers.iter().map(B::capacity).max().unwrap_or(0)
    }

    unsafe fn read_value(&self, index: usize) -> Self::Element {
        let mut result = MaybeUninit::<B::Element>::uninit_array::<SIZE>();
        for (i, buffer) in self.buffer_iter().enumerate() {
            result[i].as_mut_ptr().write(buffer.read_value(index));
        }
        MaybeUninit::array_assume_init(result)
    }

    unsafe fn write_value(&mut self, index: usize, value: Self::Element) {
        for (buffer, v) in self.buffer_iter_mut().zip(value) {
            buffer.write_value(index, v)
        }
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        for buffer in self.buffer_iter_mut() {
            buffer.manually_drop(index)
        }
    }

    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        for buffer in self.buffer_iter_mut() {
            match buffer.try_grow(target) {
                Ok(_) => {}
                Err(ResizeError::UnsupportedOperation) => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), crate::interface::ResizeError> {
        for buffer in self.buffer_iter_mut() {
            match buffer.try_shrink(target) {
                Ok(_) => {}
                Err(ResizeError::UnsupportedOperation) => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    unsafe fn manually_drop_range<R: RangeBounds<usize> + Clone>(&mut self, values_range: R) {
        for buffer in self.buffer_iter_mut() {
            buffer.manually_drop_range(values_range.clone());
        }
    }

    unsafe fn shift_right<R: RangeBounds<usize> + Clone>(&mut self, to_move: R, positions: usize) {
        for buffer in self.buffer_iter_mut() {
            buffer.shift_right(to_move.clone(), positions);
        }
    }

    unsafe fn shift_left<R: RangeBounds<usize> + Clone>(&mut self, to_move: R, positions: usize) {
        for buffer in self.buffer_iter_mut() {
            buffer.shift_left(to_move.clone(), positions);
        }
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
