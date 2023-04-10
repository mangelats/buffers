use std::marker::PhantomData;

use buffers::{base_buffers::inline::InlineBuffer, interface::Buffer};

/// Implementation of a vector
pub struct Vector<T, B: Buffer<T> = InlineBuffer<T, 1>> {
    len: usize,
    buf: B,
    _m: PhantomData<T>,
}

impl<T, B: Buffer<T>> Vector<T, B> {
    pub fn len(self) -> usize {
        self.len
    }
}

impl<T, B: Buffer<T> + Default> Vector<T, B> {
    pub fn new() -> Vector<T> {
        Vector {
            len: 0,
            buf: Default::default(),
            _m: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestVector = Vector<u32>;

    #[test]
    fn empty_vector_should_be_build_with_new() {
        let _vector = TestVector::new();
    }

    #[test]
    fn empty_vector_should_have_no_length() {
        assert_eq!(TestVector::new().len(), 0)
    }
}
