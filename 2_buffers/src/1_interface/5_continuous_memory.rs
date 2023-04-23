use std::ops::Bound::*;
use std::ops::RangeBounds;

use super::ptrs::PtrBuffer;
use super::Buffer;

/// Trait for buffers which ensures that:
///   1. All elements have an address
///   2. All the memory is allocated continuously
///
/// All common allocators actually fulfill this requirements but in some cases –like in a SoA– this may not be the case
pub trait ContinuousMemoryBuffer:
    Buffer + PtrBuffer<ConstantPointer = *const <Self as Buffer>::Element>
{
    /// Get the slice represanted by the range
    ///
    /// # SAFETY
    /// The range must be a subset of the current capacity.
    /// The memory may not be written yet, so working with it may be UB.
    unsafe fn slice<R: RangeBounds<usize>>(&self, range: R) -> &[Self::Element] {
        let (start, len) = start_len(self, range);
        std::slice::from_raw_parts(self.ptr(start), len)
    }

    /// Get the mutable slice represanted by the range
    ///
    /// # SAFETY
    /// The range must be a subset of the current capacity.
    /// The memory may not be written yet, so working with it may be UB.
    unsafe fn mut_slice<R: RangeBounds<usize>>(&mut self, range: R) -> &mut [Self::Element] {
        let (start, len) = start_len(self, range);
        std::slice::from_raw_parts_mut(self.mut_ptr(start), len)
    }
}

fn start_len<B: Buffer + ?Sized, R: RangeBounds<usize>>(buffer: &B, range: R) -> (usize, usize) {
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
