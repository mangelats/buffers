use crate::interface::{indirect_buffer::IndirectBuffer, resize_error::ResizeError, Buffer};

/// Helper (mock) buffer for testing. It passes everything to an inner buffer
/// but keeps what the last `try_grow` target was.
pub struct GrowMockBuffer<B: Buffer> {
    buff: B,
    last_target: usize,
}

impl<B: Buffer> GrowMockBuffer<B> {
    pub fn from(buff: B) -> Self {
        Self {
            buff,
            last_target: 0,
        }
    }

    pub fn last_target(&self) -> usize {
        self.last_target
    }
}

impl<B: Buffer + Default> Default for GrowMockBuffer<B> {
    fn default() -> Self {
        Self::from(Default::default())
    }
}

impl<B: Buffer> IndirectBuffer for GrowMockBuffer<B> {
    type InnerBuffer = B;
    type InnerBufferRef<'a> = &'a Self::InnerBuffer where Self: 'a;
    type InnerBufferMutRef<'a> = &'a mut Self::InnerBuffer where Self: 'a;

    fn inner(&self) -> &B {
        &self.buff
    }

    fn inner_mut(&mut self) -> &mut B {
        &mut self.buff
    }

    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        self.last_target = target;
        let inner = self.inner_mut();

        // SAFETY: Forwards call to underlying buffer.
        unsafe { inner.try_grow(target) }
    }
}
