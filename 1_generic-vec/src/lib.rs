use std::marker::PhantomData;

use buffers::interface::Buffer;

/// Implementation of a vector
pub struct Vector<T, B: Buffer<T>> {
    len: usize,
    buffer: B,
    _m: PhantomData<T>,
}

impl<T, B: Buffer<T>> Vector<T, B> {
    /// Create a new vector using the given buffer.
    ///
    /// ```rust
    /// # use buffers::base_buffers::inline::InlineBuffer;
    /// # use generic_vec::Vector;
    /// # type ExampleBuffer = InlineBuffer<u32, 1>;
    /// let _vec = Vector::from_buffer(ExampleBuffer::new());
    /// ```
    pub fn from_buffer(buffer: B) -> Vector<T, B> {
        Vector {
            len: 0,
            buffer,
            _m: PhantomData,
        }
    }

    /// Returns the number of elements currently in the Vector
    ///
    /// ```rust
    /// # use buffers::base_buffers::inline::InlineBuffer;
    /// # use generic_vec::Vector;
    /// # type ExampleBuffer = InlineBuffer<u32, 1>;
    /// let vec = Vector::<_, ExampleBuffer>::new();
    /// assert_eq!(vec.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, value: T) {
        let index = self.len;
        // TODO: check capacity
        unsafe {
            // SAFETY: we know this value is unused because of len
            self.buffer.write_value(index, value)
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> T {
        self.len -= 1;
        // TODO: check boudnaries
        unsafe { self.buffer.read_value(self.len) }
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
        assert_eq!(TestVector::new().len(), 0);
    }

    #[test]
    fn pushed_values_should_increase_len() {
        let mut vec = TestVector::new();
        assert_eq!(vec.len(), 0);

        vec.push(0);
        assert_eq!(vec.len(), 1);

        vec.push(1);
        assert_eq!(vec.len(), 2);
    }
    #[test]
    fn pushed_values_should_pop_in_reverse_order() {
        let mut vec = TestVector::new();
        vec.push(123);
        vec.push(456);

        assert_eq!(vec.pop(), 456);
        assert_eq!(vec.pop(), 123);
    }

    #[test]
    fn drops_contents_on_drop() {
        use std::sync::atomic::{AtomicI64, Ordering};

        let counter = AtomicI64::new(0);
    }
}
