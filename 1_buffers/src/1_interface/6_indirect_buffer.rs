use std::ops::{Deref, DerefMut, RangeBounds};

use crate::narrow_ref::{NarrowMutRef, NarrowRef};

use super::buffer::Buffer;
use super::contiguous_memory::ContiguousMemoryBuffer;
use super::ptrs::PtrBuffer;
use super::refs::RefBuffer;
use super::resize_error::ResizeError;

/// Trait which by default forwards all behaviour into an inner buffer. This is
/// perticularly useful to allow modifying a single function without having to
/// implement Buffer in its entirity (eg. changing the minimum that can grow).
pub trait IndirectBuffer {
    type InnerBuffer: Buffer;

    /// Utility type which is used to able to tell rust the proper lifetime of
    /// references.
    ///
    /// If you are implementing an Indirect buffer this should probably be
    /// `&'a Self::InnerBuffer where Self: 'a;` (cannot set a default value
    /// type).
    type InnerBufferRef<'a>: NarrowRef<'a, Self::InnerBuffer>
    where
        Self: 'a;

    /// Utility type which is used to able to tell rust the proper lifetime of
    /// mutable references.
    ///
    /// If you are implementing an Indirect buffer this should probably be
    /// `&'a mut Self::InnerBuffer where Self: 'a;` (cannot set a default
    /// value type).
    type InnerBufferMutRef<'a>: NarrowMutRef<'a, Self::InnerBuffer>
    where
        Self: 'a;

    fn inner(&self) -> Self::InnerBufferRef<'_>;
    fn inner_mut(&mut self) -> Self::InnerBufferMutRef<'_>;

    fn capacity(&self) -> usize {
        self.inner().narrow_ref().capacity()
    }

    unsafe fn read_value(&self, index: usize) -> <Self::InnerBuffer as Buffer>::Element {
        self.inner().narrow_ref().read_value(index)
    }

    unsafe fn write_value(&mut self, index: usize, value: <Self::InnerBuffer as Buffer>::Element) {
        self.inner_mut().narrow_mut_ref().write_value(index, value)
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        self.inner_mut().narrow_mut_ref().manually_drop(index)
    }

    unsafe fn manually_drop_range<R: RangeBounds<usize>>(&mut self, values_range: R) {
        self.inner_mut()
            .narrow_mut_ref()
            .manually_drop_range(values_range)
    }

    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        self.inner_mut().narrow_mut_ref().try_grow(target)
    }

    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError> {
        self.inner_mut().narrow_mut_ref().try_shrink(target)
    }

    unsafe fn shift_right<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {
        self.inner_mut()
            .narrow_mut_ref()
            .shift_right(to_move, positions)
    }

    unsafe fn shift_left<R: RangeBounds<usize>>(&mut self, to_move: R, positions: usize) {
        self.inner_mut()
            .narrow_mut_ref()
            .shift_left(to_move, positions)
    }
}

/// Implementation of Buffer which forwards to IndirectBuffer's methods.
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

/// Implementation of Buffer which forwards to the underlying buffer.
impl<B, IB> PtrBuffer for IB
where
    IB: IndirectBuffer<InnerBuffer = B>,
    B: Buffer + PtrBuffer,
{
    type ConstantPointer = B::ConstantPointer;
    type MutablePointer = B::MutablePointer;

    unsafe fn ptr(&self, index: usize) -> Self::ConstantPointer {
        self.inner().narrow_ref().ptr(index)
    }

    unsafe fn mut_ptr(&mut self, index: usize) -> Self::MutablePointer {
        self.inner_mut().narrow_mut_ref().mut_ptr(index)
    }
}
impl<IB> RefBuffer for IB
where
    IB: IndirectBuffer,
    IB::InnerBuffer: RefBuffer,
{
    // Forward references types to the ones in `Self::IndirectBuffer`.
    type ConstantReference<'a> = <<IB as IndirectBuffer>::InnerBuffer as RefBuffer>::ConstantReference<'a> where Self: 'a;
    type MutableReference<'a> = <<IB as IndirectBuffer>::InnerBuffer as RefBuffer>::MutableReference<'a> where Self: 'a;

    unsafe fn index<'a: 'b, 'b>(&'a self, index: usize) -> Self::ConstantReference<'b> {
        self.inner().narrow_ref().index(index)
    }

    unsafe fn mut_index<'a: 'b, 'b>(&'a mut self, index: usize) -> Self::MutableReference<'b> {
        self.inner_mut().narrow_mut_ref().mut_index(index)
    }
}
impl<IB> ContiguousMemoryBuffer for IB
where
    IB: IndirectBuffer,
    IB::InnerBuffer: ContiguousMemoryBuffer,
{
}

/// Blanket implementation to anything that can mutably dereference into a
/// buffer, as a way of forwarding. This includes `&mut T`, `Box<T>`, etc.
impl<D> IndirectBuffer for D
where
    D: DerefMut,
    D::Target: Buffer + Sized,
{
    type InnerBuffer = <D as Deref>::Target;

    type InnerBufferRef<'a> = &'a Self::InnerBuffer where Self: 'a;
    type InnerBufferMutRef<'a> = &'a mut Self::InnerBuffer where Self: 'a;

    fn inner(&self) -> Self::InnerBufferRef<'_> {
        self.deref()
    }

    fn inner_mut(&mut self) -> Self::InnerBufferMutRef<'_> {
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
