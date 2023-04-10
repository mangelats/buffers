use std::marker::PhantomData;

use buffers::base_buffers::inline::InlineBuffer;

/// Implementation of a vector
pub struct Vector<T> {
    len: usize,
    buf: InlineBuffer<T, 1234>,
    _m: PhantomData<T>,
}

impl<T> Vector<T> {
    pub fn new() -> Vector<T> {
        Vector {
            len: 0,
            buf: Default::default(),
            _m: PhantomData,
        }
    }

    pub fn len(self) -> usize {
        self.len
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
