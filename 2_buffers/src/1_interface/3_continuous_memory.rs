use std::ops::Bound::*;
use std::ops::{Range, RangeBounds};

use super::Buffer;

/// Trait for buffers which ensures that:
///   1. All elements have an address
///   2. All the memory is allocated continuously
///
/// All common allocators actually fulfill this requirements but in some cases –like in a SoA– this may not be the case
pub trait ContinuousMemoryBuffer: Buffer {
    /// Get a contant pointer to the value in the specified index.
    ///
    /// # SAFETY
    /// `index` needs to be in bounds (`0 <= index < capacity`). It's undefined behaviour when not.
    ///
    /// The pointer may point to unitialized or garbage data. It's the responsability of the caller to keep track of the state.
    unsafe fn ptr(&self, index: usize) -> *const Self::Element;

    /// Get a mutable pointer to the value in the specified index.
    ///
    /// # SAFETY
    /// `index` needs to be in bounds (`0 <= index < capacity`). It's undefined behaviour when not.
    ///
    /// The pointer may point to unitialized or garbage data. It's the responsability of the caller to keep track of the state.
    unsafe fn mut_ptr(&mut self, index: usize) -> *mut Self::Element;

    /// Get the slice represanted by the range
    ///
    /// # SAFETY
    ///
    unsafe fn slice<R: RangeBounds<usize>>(&self, range: R) -> &[Self::Element] {
        let start: usize = match range.start_bound() {
            Included(index) => *index,
            Excluded(index) => *index + 1,
            Unbounded => 0,
        };
        let end: usize = match range.end_bound() {
            Included(index) => *index,
            Excluded(index) => *index - 1,
            Unbounded => self.capacity(),
        };
        // std::slice::from_raw_parts(, len)
        todo!()
    }
}
