use std::ops::Range;

use super::resize_error::ResizeError;

/// Low level of abstraction of multiple instances of data of type `T` managed as a group.
///
/// This is perticularly useful to allow different ways of managing data in memory with a uniform interface.
///
/// ## Safety
/// Buffers are not responsible for a lot of safety features that one may expect (like dropping the values on
/// drop, check boundaries, check if the memory is initialized, and so on). This is because implementations may
/// ensure safety by design instead of adding checks every time. A lot of times the buffer doesn't have the
/// information anyways, making the check hard or impossible. In practice this makes this trait and most of its
/// methods unsafe.
///
/// ## Notes
/// This interface has been deliberately designed to have a little constrains to the implementations as possible.
/// For example: the underlying data doesn't need to be saved in a contiguous chunk of memory, and it could be on
/// the stack, on the heap, etc.
pub trait Buffer<T> {
    /// Current capacity of the buffer
    fn capacity(&self) -> usize;

    /// Reads the index position in the buffer, and empties it.
    ///
    /// # Safety
    /// The `index` position must not be empty.
    unsafe fn read_value(&self, index: usize) -> T;

    /// Writes the value into the index position of this buffer (which is no longer empty).
    ///
    /// # Safety
    /// The `index` position must not contain a value.
    unsafe fn write_value(&mut self, index: usize, value: T);

    /// Manually drops the value in the specified index position and empties it.
    ///
    /// # Safety
    /// The `index` position must not be empty.
    unsafe fn manually_drop(&mut self, index: usize);

    /// Manually drops all the values specified by the position range and empties it.
    ///
    /// By default it calls `manually_drop` one by one, but in most cases it can be overridden for a more performant
    /// version.
    ///
    /// # Safety
    /// All the positions in `values_range` must not be empty.
    unsafe fn manually_drop_range(&mut self, values_range: Range<usize>) {
        for index in values_range {
            self.manually_drop(index);
        }
    }

    /// Attempt to grow the buffer.
    ///
    /// This operation may fail a number of ways depending of the implementation and T
    ///
    /// # Safety
    /// Target size must be bigger than the current capacity (and thus, also 0)
    unsafe fn try_grow(&mut self, _target: usize) -> Result<(), ResizeError> {
        Err(ResizeError::UnsupportedOperation)
    }

    /// Attempt to shrink the buffer.
    ///
    /// This operation may fail a number of ways depending of the implementation and T
    ///
    /// # Safety
    /// Target size must be smaller than the current capacity but bigger than 0
    unsafe fn try_shrink(&mut self, _target: usize) -> Result<(), ResizeError> {
        Err(ResizeError::UnsupportedOperation)
    }
}
