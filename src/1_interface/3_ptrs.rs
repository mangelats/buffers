use super::Buffer;

/// This trait extends the buffers that have the hability to recover elements
/// from a pointer. This pointer may be a regular rust pointer (`*mut T`) but it
/// doesn't have to. For example: in a structure of arrays setting it could be a
/// structure of rust pointers.
///
/// In some cases multiple positions may have a shared pointer (eg. zero-sized
/// types in a [`crate::base_buffers::zst::ZstBuffer`])
pub trait PtrBuffer: Buffer {
    /// Type representing a constant pointer of [`Buffer::Element`].
    type ConstantPointer;

    /// Type representing a mutable pointer of [`Buffer::Element`].
    type MutablePointer;

    /// Get the constant pointer of the element in the specified position.
    ///
    /// This pointer must point to an actual value which can be dereferenced.
    ///
    /// # Safety
    ///   * `index` must be a valid position.
    ///
    /// # Notes
    /// Calling the function multiple times with the same `index` results in the
    /// same value.
    unsafe fn ptr(&self, index: usize) -> Self::ConstantPointer;

    /// Get the mutable pointer of the element in the specified position.
    ///
    /// This pointer must point to an actual value which can be dereferenced.
    ///
    /// # Safety
    ///   * `index` must be a valid position.
    ///
    /// # Notes
    /// Calling the function multiple times with the same `index` results in the
    /// same value.
    unsafe fn mut_ptr(&mut self, index: usize) -> Self::MutablePointer;
}
