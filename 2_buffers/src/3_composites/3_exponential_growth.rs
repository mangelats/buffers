use crate::interface::{buffer_mod::BufferMod, resize_error::ResizeError, Buffer};

/// Composite that grows exponentially (power of 2) instead of the actual
/// target passed.
#[repr(transparent)]
pub struct ExponentGrowthBuffer<B: Buffer>(B);

impl<B: Buffer> ExponentGrowthBuffer<B> {
    /// Make a new [`ExponentGrowthBuffer<B>`] given `B`.
    pub fn from(b: B) -> Self {
        Self(b)
    }
}

impl<B: Buffer + Default> Default for ExponentGrowthBuffer<B> {
    fn default() -> Self {
        Self::from(Default::default())
    }
}

impl<B: Buffer> BufferMod for ExponentGrowthBuffer<B> {
    type InnerBuffer = B;

    fn inner(&self) -> &Self::InnerBuffer {
        &self.0
    }

    fn inner_mut(&mut self) -> &mut Self::InnerBuffer {
        &mut self.0
    }

    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        // SAFETY: target is always bigger than 0 because of the restriction on Buffer; it won't underflow.
        let new_target = (target - 1).next_power_of_two();
        self.inner_mut().try_grow(new_target)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        base_buffers::inline::InlineBuffer, composites::grow_mock::GrowMockBuffer,
        interface::Buffer,
    };

    use super::ExponentGrowthBuffer;

    #[test]
    fn test_properly_growing() {
        let mut mock_buffer: GrowMockBuffer<InlineBuffer<u32, 1>> = Default::default();
        {
            let mut buffer = ExponentGrowthBuffer::from(&mut mock_buffer);
            // This will fail, but it doesn't matter for this test.
            let _ = unsafe { buffer.try_grow(10) };
        }
        assert_eq!(mock_buffer.last_target(), 16);
    }
}
