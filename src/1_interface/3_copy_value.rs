use super::Buffer;

pub trait CopyValueBuffer: Buffer
where
    Self::Element: Copy,
{
    /// Copies the value in `position` without emptying it.
    ///
    /// Unlike [`Buffer::read_value`] it doesn't require mutable access because
    /// it doesn't empty the position.
    ///
    /// # Safety
    ///   * `index` must be less than `capacity`.
    ///   * The `index` position must be filled.
    unsafe fn copy(&self, index: usize) -> Self::Element;
}
