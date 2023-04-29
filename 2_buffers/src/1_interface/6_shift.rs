use std::ops::Bound::*;
use std::ops::RangeBounds;

use super::continuous_memory::ContinuousMemoryBuffer;
use super::Buffer;

/// Trait that defines how to shift values in the buffer.
///
/// This are usually used to `insert` or `remove` values from the middle of the buffer.
pub trait BufferShift: Buffer {
    /// Shift a range of values to the right.
    /// # Safety
    /// The values must exist and the new location should be itself or an empty spot
    ///
    /// There should be enough space to the right
    unsafe fn shift_right<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize);

    /// Shift a range of values to the left.
    ///
    /// # Safety
    /// The values must exist and the new location should be itself or an empty spot
    ///
    /// There should be enough space to the left
    unsafe fn shift_left<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize);
}

trait BufferShiftImpl {
    type B: Buffer + ?Sized;
    unsafe fn shift_right<R: RangeBounds<usize>>(buff: &mut Self::B, to_move: R, positions: usize);
    unsafe fn shift_left<R: RangeBounds<usize>>(buff: &mut Self::B, to_move: R, positions: usize);
}

trait ForwardBufferShiftImpl {
    type Impl: BufferShiftImpl;
}
