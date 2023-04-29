use std::marker::PhantomData;
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

pub trait ForwardBufferShiftImpl: Buffer {
    type Impl: BufferShiftImpl<Buff = Self>;
}
impl<B: ForwardBufferShiftImpl + ?Sized> BufferShift for B {
    unsafe fn shift_right<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {
        <Self as ForwardBufferShiftImpl>::Impl::shift_right(self, to_move, positions)
    }

    unsafe fn shift_left<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {
        <Self as ForwardBufferShiftImpl>::Impl::shift_left(self, to_move, positions)
    }
}

pub trait BufferShiftImpl {
    type Buff: Buffer + ?Sized;
    unsafe fn shift_right<R: RangeBounds<usize>>(
        buff: &mut Self::Buff,
        to_move: R,
        positions: usize,
    );
    unsafe fn shift_left<R: RangeBounds<usize>>(
        buff: &mut Self::Buff,
        to_move: R,
        positions: usize,
    );
}

pub struct ShiftInBlock<B: Buffer + ?Sized>(PhantomData<B>);
impl<B: Buffer + ?Sized> BufferShiftImpl for ShiftInBlock<B> {
    type Buff = B;

    unsafe fn shift_right<R: RangeBounds<usize>>(
        buff: &mut Self::Buff,
        to_move: R,
        positions: usize,
    ) {
        let (start, end) = start_end(buff, to_move);

        let size = end - start;
        let new_end = end + positions;

        debug_assert!(new_end < buff.capacity());

        for current in 0..size {
            let new_pos = new_end - current;
            let old_pos = end - current;
            buff.write_value(new_pos, buff.read_value(old_pos));
        }

        // Old values left as is, since the bytes themselves are considered garbage
    }

    unsafe fn shift_left<R: RangeBounds<usize>>(
        buff: &mut Self::Buff,
        to_move: R,
        positions: usize,
    ) {
        let (start, end) = start_end(buff, to_move);

        debug_assert!(start >= positions);

        let size = end - start;
        let new_start = start - positions;

        for current in 0..size {
            let new_pos = new_start + current;
            let old_pos = start + current;
            buff.write_value(new_pos, buff.read_value(old_pos));
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
