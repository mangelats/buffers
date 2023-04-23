use std::ops::RangeBounds;

use super::Buffer;

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

pub trait ShiftOneByOne: Buffer {}
impl<T: ShiftOneByOne> BufferShift for T {
    unsafe fn shift_right<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {
        todo!()
    }

    unsafe fn shift_left<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {
        todo!()
    }
}
