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

#[derive(Copy, Clone)]
pub enum BufferShiftAlg {
    ShiftInBlock = 1,
    ShiftOneByOne = 2,
}

pub trait ShiftImpl<const alg: u8>: Buffer {}

// /// Automatically implement `BufferShift` by copying the values one by one.
// ///
// /// This should work for any buffer, but some can have an optimized version (see `ShiftInBlock`).
// pub trait ShiftInBlock: ContinuousMemoryBuffer {}
// impl<T: ShiftInBlock> BufferShift for T {
//     unsafe fn shift_right<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {
//         let (start, end) = start_end(self, to_move);

//         let size = end - start;
//         let new_end = end + positions;

//         debug_assert!(new_end < self.capacity());

//         for current in 0..size {
//             let new_pos = new_end - current;
//             let old_pos = end - current;
//             self.write_value(new_pos, self.read_value(old_pos));
//         }

//         // Old values left as is, since the bytes themselves are considered garbage
//     }

//     unsafe fn shift_left<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {
//         let (start, end) = start_end(self, to_move);

//         debug_assert!(start >= positions);

//         let size = end - start;
//         let new_start = start - positions;

//         for current in 0..size {
//             let new_pos = new_start + current;
//             let old_pos = start + current;
//             self.write_value(new_pos, self.read_value(old_pos));
//         }

//         // Old values left as is, since the bytes themselves are considered garbage
//     }
// }

/// Automatically implement `BufferShift` by copying the values one by one.
///
/// This should work for any buffer, but some can have an optimized version (see `ShiftInBlock`).
const SHIFT_BY_ONE: u8 = BufferShiftAlg::ShiftOneByOne as u8;
impl<T: ShiftImpl<SHIFT_BY_ONE>> BufferShift for T {
    unsafe fn shift_right<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {
        let (start, end) = start_end(self, to_move);

        let size = end - start;
        let new_end = end + positions;

        debug_assert!(new_end < self.capacity());

        for current in 0..size {
            let new_pos = new_end - current;
            let old_pos = end - current;
            self.write_value(new_pos, self.read_value(old_pos));
        }

        // Old values left as is, since the bytes themselves are considered garbage
    }

    unsafe fn shift_left<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {
        let (start, end) = start_end(self, to_move);

        debug_assert!(start >= positions);

        let size = end - start;
        let new_start = start - positions;

        for current in 0..size {
            let new_pos = new_start + current;
            let old_pos = start + current;
            self.write_value(new_pos, self.read_value(old_pos));
        }

        // Old values left as is, since the bytes themselves are considered garbage
    }
}

fn start_end<B: Buffer + ?Sized, R: RangeBounds<usize>>(buffer: &B, range: R) -> (usize, usize) {
    let start: usize = match range.start_bound() {
        Included(index) => *index,
        Excluded(index) => *index + 1,
        Unbounded => 0,
    };
    let end: usize = match range.end_bound() {
        Included(index) => *index + 1,
        Excluded(index) => *index,
        Unbounded => buffer.capacity(),
    };
    (start, end)
}
