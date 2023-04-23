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
    /// `index` needs to be in bounds (`0 <= index < SIZE`). It's undefined behaviour when not.
    ///
    /// The pointer may point to unitialized or garbage data. It's the responsability of the caller to keep track of the state.
    unsafe fn ptr(&self, index: usize) -> *const Self::Element;

    /// Get a mutable pointer to the value in the specified index.
    ///
    /// # SAFETY
    /// `index` needs to be in bounds (`0 <= index < SIZE`). It's undefined behaviour when not.
    ///
    /// The pointer may point to unitialized or garbage data. It's the responsability of the caller to keep track of the state.
    unsafe fn mut_ptr(&mut self, index: usize) -> *mut Self::Element;
}
