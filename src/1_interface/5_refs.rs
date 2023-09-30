use super::Buffer;

/// This trait extends the buffers that have the hability to generate references
/// to an element in the buffer. This reference may be a regular rust reference
/// (`&T`) but it doesn't have to. For example: in a structure of arrays setting
/// it could be a structure of references.
pub trait RefBuffer: Buffer {
    /// Type representing a reference to [`Buffer::Element`] with `'a` lifetime.
    type ConstantReference<'a>
    where
        Self: 'a;

    /// Type representing a mutable reference to [`Buffer::Element`] with `'a`
    /// lifetime.
    type MutableReference<'a>
    where
        Self: 'a;

    /// Get a reference to the element in the specified position.
    ///
    /// # Safety
    ///   * `index` must be a valid position.
    ///   * Position `index` must be filled.
    unsafe fn index<'a: 'b, 'b>(&'a self, index: usize) -> Self::ConstantReference<'b>;

    /// Get a mutable reference to the element in the specified position.
    ///
    /// # Safety
    ///   * `index` must be a valid position.
    ///   * Position `index` must be filled.
    unsafe fn mut_index<'a: 'b, 'b>(&'a mut self, index: usize) -> Self::MutableReference<'b>;
}
