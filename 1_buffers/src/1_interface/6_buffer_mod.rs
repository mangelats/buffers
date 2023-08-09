#[cfg(feature = "allocator")]
use std::alloc::Allocator;
use std::ops::RangeBounds;

use super::buffer::Buffer;
use super::resize_error::ResizeError;

/// Trait which by default forwards all behaviour into an inner buffer. This is
/// perticularly useful to allow modifying a single function without having to
/// implement Buffer in its entirity (eg. changing the minimum that can grow).
pub trait BufferMod {
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

impl<T: BufferMod> Buffer for T {
    type Element = <<Self as BufferMod>::InnerBuffer as Buffer>::Element;

    fn capacity(&self) -> usize {
        <Self as BufferMod>::capacity(self)
    }

    unsafe fn read_value(&self, index: usize) -> Self::Element {
        <Self as BufferMod>::read_value(self, index)
    }

    unsafe fn write_value(&mut self, index: usize, value: Self::Element) {
        <Self as BufferMod>::write_value(self, index, value)
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        <Self as BufferMod>::manually_drop(self, index)
    }

    unsafe fn manually_drop_range<R: RangeBounds<usize>>(&mut self, values_range: R) {
        <Self as BufferMod>::manually_drop_range(self, values_range)
    }

    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        <Self as BufferMod>::try_grow(self, target)
    }

    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError> {
        <Self as BufferMod>::try_shrink(self, target)
    }

    unsafe fn shift_right<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {
        <Self as BufferMod>::shift_right(self, to_move, positions)
    }

    unsafe fn shift_left<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {
        <Self as BufferMod>::shift_left(self, to_move, positions)
    }
}

#[cfg(feature = "allocator")]
impl<B: Buffer, A: Allocator> BufferMod for Box<B, A> {
    type InnerBuffer = B;

    fn inner(&self) -> &Self::InnerBuffer {
        &**self
    }

    fn inner_mut(&mut self) -> &mut Self::InnerBuffer {
        &mut **self
    }
}

#[cfg(not(feature = "allocator"))]
impl<B: Buffer> BufferMod for Box<B> {
    type InnerBuffer = B;

    fn inner(&self) -> &Self::InnerBuffer {
        &**self
    }

    fn inner_mut(&mut self) -> &mut Self::InnerBuffer {
        &mut **self
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
