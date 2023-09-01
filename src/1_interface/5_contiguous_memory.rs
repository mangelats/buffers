use std::ops::Bound::*;
use std::ops::RangeBounds;

use super::ptrs::PtrBuffer;
use super::Buffer;

/// Trait that marks a buffer and means that it saves all data in a contiguous
/// memory block. It also adds utility functions based on that fact.
///
/// To be such buffer it must ensure that:
///   1. All elements have a distinct pointer.
///   2. All the memory is allocated contiguously, following an array layout.
///
/// This is quite common but it cannot be assumed in the base trait.
pub trait ContiguousMemoryBuffer:
    Buffer
    + PtrBuffer<
        ConstantPointer = *const <Self as Buffer>::Element,
        MutablePointer = *mut <Self as Buffer>::Element,
    >
{
    /// Get the slice of memory of the buffer specified by `range`.
    ///
    /// # Safety
    ///  * `range` must be a range of valid positions.
    ///  * All positions in `range` must be filled.
    unsafe fn slice<R: RangeBounds<usize> + Clone>(&self, range: R) -> &[Self::Element] {
        let (start, len) = start_len(self, range);
        std::slice::from_raw_parts(self.ptr(start), len)
    }

    /// Get the mutable slice of memory of the buffer specified by `range`.
    ///
    /// # Safety
    ///  * `range` must be a range of valid positions.
    ///  * All positions in `range` must be filled.
    unsafe fn mut_slice<R: RangeBounds<usize> + Clone>(
        &mut self,
        range: R,
    ) -> &mut [Self::Element] {
        let (start, len) = start_len(self, range);
        std::slice::from_raw_parts_mut(self.mut_ptr(start), len)
    }
}

/// Finds the start and length of a range for a specific buffer (allows open
/// ranges).
fn start_len<B: Buffer + ?Sized, R: RangeBounds<usize> + Clone>(
    buffer: &B,
    range: R,
) -> (usize, usize) {
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

    let size = if start <= end { 0 } else { end - start };

    (start, size)
}
