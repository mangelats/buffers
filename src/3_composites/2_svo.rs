use std::ops::RangeBounds;

use crate::{
    base_buffers::inline::InlineBuffer,
    interface::{
        contiguous_memory::ContiguousMemoryBuffer, ptrs::PtrBuffer, refs::RefBuffer,
        resize_error::ResizeError, Buffer,
    },
};

use super::either::EitherBuffer;

/// Buffer composite that adds small vector optimization (SVO) to a given
/// buffer. This means that it starts working with an inline buffer (which is
/// usually left on the stack) but can automatically grow into an arbitrary
/// bigger buffer (usually a heap-allocated one which can grow).
pub struct SvoBuffer<const SMALL_SIZE: usize, B>
where
    B: ContiguousMemoryBuffer + Default,
{
    inner: EitherBuffer<InlineBuffer<B::Element, SMALL_SIZE>, B>,
}

impl<const SMALL_SIZE: usize, B> SvoBuffer<SMALL_SIZE, B>
where
    B: ContiguousMemoryBuffer + Default,
{
    /// Creates a new empty buffer
    pub fn new() -> Self {
        Default::default()
    }

    /// Internal only.
    ///
    /// Move all data from the small vector into the big one.
    ///
    /// # SAFETY
    ///   * `target` > `SMALL_SIZE`
    unsafe fn move_into_big(&mut self, target: usize) -> Result<(), ResizeError> {
        let EitherBuffer::First(ref current_buf) = self.inner else {
            // SAFETY: This is only called when we grow from small to big.
            // This means that we always have an inline buffer at this point
            unreachable!()
        };

        let mut new_buf: B = Default::default();
        if new_buf.capacity() < target {
            // SAFETY: The conditional checks that `new_buffer` actually needs
            // to grow.
            unsafe { new_buf.try_grow(target)? };
        }

        // SAFETY: `current_buf.capacity()` > 0; thus `0` is a valid index.
        let src = unsafe { current_buf.ptr(0) };
        // SAFETY: `new_buf.capacity()` > 0; thus `0` is a valid index.
        let dst = unsafe { new_buf.mut_ptr(0) };

        // SAFETY:
        //   * Both buffers have contiguous memory.
        //   * `new_buf.capacity()` > `current_buf.capacity()`.
        //   * They do not reuse the same memory.
        unsafe { std::ptr::copy_nonoverlapping(src, dst, current_buf.capacity()) };

        self.inner = EitherBuffer::Second(new_buf);
        Ok(())
    }
}

impl<const SMALL_SIZE: usize, B> Default for SvoBuffer<SMALL_SIZE, B>
where
    B: ContiguousMemoryBuffer + Default,
{
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<const SMALL_SIZE: usize, B> Buffer for SvoBuffer<SMALL_SIZE, B>
where
    B: ContiguousMemoryBuffer + Default,
{
    type Element = B::Element;

    fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    unsafe fn read_value(&mut self, index: usize) -> Self::Element {
        // SAFETY: Forwarding call to inner buffer.
        unsafe { self.inner.read_value(index) }
    }

    unsafe fn write_value(&mut self, index: usize, value: Self::Element) {
        // SAFETY: Forwarding call to inner buffer.
        unsafe { self.inner.write_value(index, value) }
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        // SAFETY: Forwarding call to inner buffer.
        unsafe { self.inner.manually_drop(index) }
    }

    unsafe fn manually_drop_range<R: RangeBounds<usize> + Clone>(&mut self, values_range: R) {
        // SAFETY: Forwarding call to inner buffer.
        unsafe { self.inner.manually_drop_range(values_range) }
    }
    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        match self.inner {
            EitherBuffer::First(_) => {
                // SAFETY: `target` > `self.capacity()` = `SMALL_SIZE`
                unsafe { self.move_into_big(target) }
            }
            EitherBuffer::Second(ref mut buf) => {
                // SAFETY: Forwarding call to big buffer.
                unsafe { buf.try_grow(target) }
            }
        }
    }
    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError> {
        match self.inner {
            EitherBuffer::First(_) => Ok(()),
            EitherBuffer::Second(ref mut buf) => {
                // SAFETY: Forwarding call to big buffer.
                unsafe { buf.try_shrink(target) }
            }
        }
    }
}

impl<const SMALL_SIZE: usize, B> PtrBuffer for SvoBuffer<SMALL_SIZE, B>
where
    B: ContiguousMemoryBuffer + Default,
{
    type ConstantPointer = B::ConstantPointer;
    type MutablePointer = B::MutablePointer;

    unsafe fn ptr(&self, index: usize) -> *const Self::Element {
        // SAFETY: Forwarding call to inner buffer.
        unsafe { self.inner.ptr(index) }
    }

    unsafe fn mut_ptr(&mut self, index: usize) -> *mut Self::Element {
        // SAFETY: Forwarding call to inner buffer.
        unsafe { self.inner.mut_ptr(index) }
    }
}

impl<const SMALL_SIZE: usize, B> RefBuffer for SvoBuffer<SMALL_SIZE, B>
where
    B: ContiguousMemoryBuffer + Default,
    for<'a> B: RefBuffer<
            ConstantReference<'a> = &'a <B as Buffer>::Element,
            MutableReference<'a> = &'a mut <B as Buffer>::Element,
        > + 'a,
{
    type ConstantReference<'a> = B::ConstantReference<'a>;

    type MutableReference<'a> = B::MutableReference<'a>;

    unsafe fn index<'a: 'b, 'b>(&'a self, index: usize) -> Self::ConstantReference<'b> {
        // Borrow checker can't check `self.inner.index(index)` lifetimes.
        match self.inner {
            EitherBuffer::First(ref b) => {
                // SAFETY: Forwarding call to small buffer.
                unsafe { RefBuffer::index(b, index) }
            }
            EitherBuffer::Second(ref b) => {
                // SAFETY: Forwarding call to big buffer.
                unsafe { RefBuffer::index(b, index) }
            }
        }
    }

    unsafe fn mut_index<'a: 'b, 'b>(&'a mut self, index: usize) -> Self::MutableReference<'b> {
        // Borrow checker can't check `self.inner.mut_index(index)` lifetimes.
        match self.inner {
            EitherBuffer::First(ref mut b) => {
                // SAFETY: Forwarding call to small buffer.
                unsafe { RefBuffer::mut_index(b, index) }
            }
            EitherBuffer::Second(ref mut b) => {
                // SAFETY: Forwarding call to big buffer.
                unsafe { RefBuffer::mut_index(b, index) }
            }
        }
    }
}

impl<const SMALL_SIZE: usize, B> ContiguousMemoryBuffer for SvoBuffer<SMALL_SIZE, B> where
    B: ContiguousMemoryBuffer + Default
{
}

#[cfg(test)]
mod tests {
    use crate::base_buffers::heap::HeapBuffer;

    use super::*;

    #[test]
    fn should_be_able_to_grow() {
        let mut buffer: SvoBuffer<1, HeapBuffer<u32>> = Default::default();
        assert_eq!(buffer.capacity(), 1);
        unsafe { buffer.try_grow(32) }.expect("Should be able to grow");
        assert!(buffer.capacity() >= 32)
    }

    #[test]
    fn should_move_elements_when_growing() {
        let mut buffer: SvoBuffer<1, HeapBuffer<u32>> = Default::default();
        unsafe {
            buffer.write_value(0, 123);
            buffer.try_grow(32).expect("Should be able to grow");
            assert_eq!(buffer.read_value(0), 123);
        }
    }
}
