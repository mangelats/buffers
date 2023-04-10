use std::marker::PhantomData;

use buffers::interface::Buffer;

/// Implementation of a vector
pub struct Vector<T, B: Buffer<T>> {
    len: usize,
    buffer: B,
    _m: PhantomData<T>,
}

impl<T, B: Buffer<T>> Vector<T, B> {
    pub fn from_buffer(buffer: B) -> Vector<T, B> {
        Vector {
            len: 0,
            buffer,
            _m: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T, B: Buffer<T> + Default> Vector<T, B> {
    pub fn new() -> Vector<T, B> {
        Self::from_buffer(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use buffers::base_buffers::inline::InlineBuffer;

    use super::*;

    type TestVector = Vector<u32, InlineBuffer<u32, 4>>;

    #[test]
    fn empty_vector_should_be_build_with_new() {
        let _vector = TestVector::new();
    }

    #[test]
    fn empty_vector_should_have_no_length() {
        assert_eq!(TestVector::new().len(), 0)
    }
}
