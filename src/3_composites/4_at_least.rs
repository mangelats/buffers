use std::cmp::max;

use crate::interface::{indirect_buffer::IndirectBuffer, resize_error::ResizeError, Buffer};

/// Composite that ensures that when trying to grow it has at least a value.
/// The initial status may still be under this value and you may shrink lower
/// than it.
#[repr(transparent)]
pub struct AtLeastBuffer<const MIN_SIZE: usize, B: Buffer>(B);

impl<const MIN_SIZE: usize, B: Buffer> AtLeastBuffer<MIN_SIZE, B> {
    /// Make a new [`AtLeastBuffer<MIN_SIZE, B>`] given `B`.
    /// Note that you should specify `MIN_SIZE` in the typing.
    pub fn from(buff: B) -> Self {
        Self(buff)
    }
}

impl<const MIN_SIZE: usize, B: Buffer + Default> Default for AtLeastBuffer<MIN_SIZE, B> {
    fn default() -> Self {
        Self::from(Default::default())
    }
}

impl<const MIN_SIZE: usize, B: Buffer> IndirectBuffer for AtLeastBuffer<MIN_SIZE, B> {
    type InnerBuffer = B;
    type InnerBufferRef<'a> = &'a Self::InnerBuffer where Self: 'a;
    type InnerBufferMutRef<'a> = &'a mut Self::InnerBuffer where Self: 'a;

    fn inner(&self) -> &B {
        &self.0
    }

    fn inner_mut(&mut self) -> &mut B {
        &mut self.0
    }

    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        let inner = self.inner_mut();
        let new_target = max(target, MIN_SIZE);

        // SAFETY: `new_target` >= `target` > `self.capacity()`.
        unsafe { inner.try_grow(new_target) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        base_buffers::inline::InlineBuffer, composites::grow_mock::GrowMockBuffer,
        interface::Buffer,
    };

    use super::AtLeastBuffer;

    #[test]
    fn test_properly_growing() {
        let mut mock_buffer: GrowMockBuffer<InlineBuffer<u32, 1>> = Default::default();
        {
            let mut buffer: AtLeastBuffer<14, _> = AtLeastBuffer::from(&mut mock_buffer);
            // This will fail, but it doesn't matter for this test.
            let _ = unsafe { buffer.try_grow(3) };
        }
        assert_eq!(mock_buffer.last_target(), 14);
    }
}
