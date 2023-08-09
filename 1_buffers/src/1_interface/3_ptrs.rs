use super::Buffer;

/// Represents a buffer which has pointers to its elements
///
/// Note that in some cases the elements themselves may not ahve a unique pointers (eg. zero-sized types)
pub trait PtrBuffer: Buffer {
    type ConstantPointer;
    type MutablePointer;

    /// Get a contant pointer to the value in the specified index.
    ///
    /// # Safety
    /// `index` needs to be in bounds (`0 <= index < capacity`). It's undefined behaviour when not.
    ///
    /// The pointer may point to unitialized or garbage data. It's the responsability of the caller to keep track of the state.
    unsafe fn ptr(&self, index: usize) -> Self::ConstantPointer;

    /// Get a mutable pointer to the value in the specified index.
    ///
    /// # Safety
    /// `index` needs to be in bounds (`0 <= index < capacity`). It's undefined behaviour when not.
    ///
    /// The pointer may point to unitialized or garbage data. It's the responsability of the caller to keep track of the state.
    unsafe fn mut_ptr(&mut self, index: usize) -> Self::MutablePointer;
}
