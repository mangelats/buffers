use std::ops::{DerefMut, RangeBounds};

use super::buffer::Buffer;
use super::resize_error::ResizeError;

/// Trait which by default forwards all behaviour into an inner buffer. This is
/// perticularly useful to allow modifying a single function without having to
/// implement Buffer in its entirity (eg. changing the minimum that can grow).
pub trait IndirectBuffer {
    type InnerBuffer: Buffer;
    fn inner(&self) -> &Self::InnerBuffer;
    fn inner_mut(&mut self) -> &mut Self::InnerBuffer;

    fn capacity(&self) -> usize {
        self.inner().capacity()
    }

    unsafe fn read_value(&self, index: usize) -> <Self::InnerBuffer as Buffer>::Element {
        self.inner().read_value(index)
    }

    unsafe fn write_value(&mut self, index: usize, value: <Self::InnerBuffer as Buffer>::Element) {
        self.inner_mut().write_value(index, value)
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        self.inner_mut().manually_drop(index)
    }

    unsafe fn manually_drop_range<R: RangeBounds<usize>>(&mut self, values_range: R) {
        self.inner_mut().manually_drop_range(values_range)
    }

    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        self.inner_mut().try_grow(target)
    }

    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError> {
        self.inner_mut().try_shrink(target)
    }

    unsafe fn shift_right<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {
        self.inner_mut().shift_right(to_move, positions)
    }

    unsafe fn shift_left<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {
        self.inner_mut().shift_left(to_move, positions)
    }
}

impl<T: IndirectBuffer> Buffer for T {
    type Element = <<Self as IndirectBuffer>::InnerBuffer as Buffer>::Element;

    fn capacity(&self) -> usize {
        <Self as IndirectBuffer>::capacity(self)
    }

    unsafe fn read_value(&self, index: usize) -> Self::Element {
        <Self as IndirectBuffer>::read_value(self, index)
    }

    unsafe fn write_value(&mut self, index: usize, value: Self::Element) {
        <Self as IndirectBuffer>::write_value(self, index, value)
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        <Self as IndirectBuffer>::manually_drop(self, index)
    }

    unsafe fn manually_drop_range<R: RangeBounds<usize>>(&mut self, values_range: R) {
        <Self as IndirectBuffer>::manually_drop_range(self, values_range)
    }

    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        <Self as IndirectBuffer>::try_grow(self, target)
    }

    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError> {
        <Self as IndirectBuffer>::try_shrink(self, target)
    }

    unsafe fn shift_right<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {
        <Self as IndirectBuffer>::shift_right(self, to_move, positions)
    }

    unsafe fn shift_left<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {
        <Self as IndirectBuffer>::shift_left(self, to_move, positions)
    }
}

/// Blanket implementation to anything that can mutably dereference into a
/// buffer, as a way of forwarding. This includes `&mut T`, `Box<T>`, etc.
impl<B, D> IndirectBuffer for D
where
    B: Buffer,
    D: DerefMut<Target = B>,
{
    type InnerBuffer = B;

    fn inner(&self) -> &Self::InnerBuffer {
        self.deref()
    }

    fn inner_mut(&mut self) -> &mut Self::InnerBuffer {
        self.deref_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::Buffer;

    #[test]
    fn box_forwards_buffer() {
        use crate::base_buffers::inline::InlineBuffer;

        let b = Box::new(InlineBuffer::<u32, 10>::new());
        assert_eq!(b.capacity(), 10);
    }
}
