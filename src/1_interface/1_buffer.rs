use std::ops::Bound::*;
use std::ops::Range;
use std::ops::RangeBounds;

use super::resize_error::ResizeError;

/// Trait that represents a layout of data for a collection. This abstraction is
/// very low level and only manages the "space" itself, and not the values which
/// is the collections' responsibility.
///
/// This is the minimal trait to form a buffer. Other traits expand on it
/// allowing more operations and optimizations.
///
/// It also contains some utility methods with default implementation but in
/// some cases a more efficient one can be provided.
///
/// ## Safety
/// Because this trait isn't meant to manage what values are being saved, the
/// user has to manage it. This makes a lot of methods `unsafe`, as the buffer
/// itself cannot verify that what is being done is actually safe. This actually
/// makes this abstraction better as a lot of times the collections need to do
/// so anyways, so making them responsible simplifies it.
///
/// The jist of it is:
///   * Positions in the range `0..capacity` are considered valid.
///   * To write a value in a position, that position must be valid and empty
///     (and becomes filled).
///   * To read a value in a position, that position must be valid and filled
///     (and becomes empty).
///   * To drop a value in a position, that position must be valid and filled
///     (and becomes empty).
///   * Before droping a buffer, all positions must be and empty.
pub trait Buffer {
    /// Type of elements this buffer holds.
    type Element;

    /// How many elements can this buffer contain.
    fn capacity(&self) -> usize;

    /// Reads the `index` position in the buffer, emptying it.
    ///
    /// # Safety
    ///   * `index` must be less than `capacity`.
    ///   * The `index` position must be filled.
    unsafe fn read_value(&mut self, index: usize) -> Self::Element;

    /// Writes the value into the `index` position, filling it.
    ///
    /// # Safety
    ///   * `index` must be less than `capacity`.
    ///   * The `index` position must be empty.
    unsafe fn write_value(&mut self, index: usize, value: Self::Element);

    /// Manually drops the value in the specified index position and empties it.
    ///
    /// Unlike [`read_value`], it does not return the element, which may allow
    /// to drop it in place.
    ///
    /// # Safety
    ///   * `index` must be less than `capacity`.
    ///   * The `index` position must be filled.
    unsafe fn manually_drop(&mut self, index: usize);

    /// Asks the buffer to grow.
    ///
    /// This operation may fail a number of ways depending on the implementation
    /// and `Self::Element`. See [`ResizeError`] for more details.
    ///
    /// # Safety
    ///   * Target size must be bigger than the current capacity (and thus, also
    ///     bigger than zero)
    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError>;

    /// Asks the buffer to shrink.
    ///
    /// This operation may fail a number of ways depending on the implementation
    /// and `Self::Element`. See [`ResizeError`] for more details.
    ///
    /// # Safety
    ///  * Target size must be smaller than the current capacity.
    ///  * Positions from `target` to `capacity` must be empty.
    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError>;

    /// Utility method which drops elements (and thus empties) a range of
    /// positions.
    ///
    /// # Safety
    ///   * All the positions in `values_range` must be valid and filled.
    unsafe fn manually_drop_range<R: RangeBounds<usize> + Clone>(&mut self, values_range: R) {
        for index in clamp_buffer_range(self, values_range) {
            // SAFETY: All positions should fulfill the requirements as per
            // this function documentation.
            unsafe { self.manually_drop(index) };
        }
    }

    /// Utility method to move elements to the right by `positions`.
    ///
    /// # Safety
    ///   * All positions in `to_move` must be valid.
    ///   * `positions` positions after the `to_move` range must be valid and
    ///     empty.
    unsafe fn shift_right<R: RangeBounds<usize> + Clone>(&mut self, to_move: R, positions: usize) {
        let range = clamp_buffer_range(self, to_move);

        debug_assert!(range.end + positions <= self.capacity());

        for old_pos in range.into_iter().rev() {
            let new_pos = old_pos + positions;
            // SAFETY: This function requirements ensure that `to_move` (`range`
            // after clamp) has all values be valid. We are moving values before
            // overriding, ensuring that the value is still valid.
            let value = unsafe { self.read_value(old_pos) };
            // SAFETY: This function requirements ensure that `positions` won't
            // get out of memory empty. On the overlapping space, the values are
            // emptied before writing on it.
            unsafe { self.write_value(new_pos, value) };
        }

        // Old values left as is, since the bytes themselves are considered garbage
    }

    /// Utility method to move elements to the left by `positions`.
    ///
    /// # Safety
    ///   * All positions in `to_move` must be valid.
    ///   * `positions` positions before the `to_move` range must be valid and
    ///     empty.
    unsafe fn shift_left<R: RangeBounds<usize> + Clone>(&mut self, to_move: R, positions: usize) {
        let range = clamp_buffer_range(self, to_move);

        debug_assert!(range.end >= positions);

        for old_pos in range.into_iter() {
            let new_pos = old_pos - positions;
            // SAFETY: This function requirements ensure that `to_move` (`range`
            // after clamp) has all values be valid. We are moving values before
            // overriding, ensuring that the value is still valid.
            let value = unsafe { self.read_value(old_pos) };
            // SAFETY: This function requirements ensure that `positions` won't
            // get out of memory empty. On the overlapping space, the values are
            // emptied before writing on it.
            unsafe { self.write_value(new_pos, value) };
        }

        // Old values left as is, since the bytes themselves are considered garbage
    }
}

/// Utility function that clamps a range into a buffer cappacity. Allows for
/// open ended ranges in the ranged utility functions.
fn clamp_buffer_range<B: Buffer + ?Sized, R: RangeBounds<usize> + Clone>(
    buffer: &B,
    range: R,
) -> Range<usize> {
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
    start..end
}
