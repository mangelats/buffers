use std::marker::PhantomData;

use buffers::{
    interface::{continuous_memory::ContinuousMemoryBuffer, ptrs::PtrBuffer, Buffer},
    DefaultBuffer,
};

/// Implementation of a vector
pub struct Vector<T, B: Buffer<Element = T> = DefaultBuffer<T>> {
    len: usize,
    buffer: B,
    _m: PhantomData<T>,
}

impl<T, B: Buffer<Element = T>> Vector<T, B> {
    /// Create a new vector using the given buffer.
    ///
    /// ```
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
    /// ```
    /// # use buffers::base_buffers::inline::InlineBuffer;
    /// # use generic_vec::Vector;
    /// # type ExampleBuffer = InlineBuffer<u32, 1>;
    /// let vec = Vector::<_, ExampleBuffer>::new();
    /// assert_eq!(vec.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Queries the buffer for its capacity
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    /// Tries to add a value at the end of the vector. This may fail if there is not enough
    /// space and the buffer cannot grow.
    ///
    /// ```
    /// # use buffers::base_buffers::inline::InlineBuffer;
    /// # use generic_vec::Vector;
    /// # type ExampleBuffer = InlineBuffer<u32, 1>;
    /// let mut vec = Vector::<u32, ExampleBuffer>::new();
    /// vec.try_push(1);
    /// let length = vec.len(); // Length is 1
    /// # assert_eq!(length, 1);
    /// ```
    pub fn try_push(&mut self, value: T) -> Result<usize, ()> {
        let index = self.len;
        if index >= self.buffer.capacity() {
            unsafe {
                self.buffer.try_grow(self.len + 1).map_err(|_| ())?;
            }
        }
        unsafe {
            // SAFETY: we know this value is unused because of len
            self.buffer.write_value(index, value)
        }
        self.len += 1;
        Ok(index)
    }

    /// Adds a value at the end of the vector. Panics if it cannot.
    ///
    /// ```
    /// # use buffers::base_buffers::inline::InlineBuffer;
    /// # use generic_vec::Vector;
    /// # type ExampleBuffer = InlineBuffer<u32, 1>;
    /// let mut vec = Vector::<u32, ExampleBuffer>::new();
    /// vec.push(1);
    /// let length = vec.len(); // Length is 1
    /// # assert_eq!(length, 1);
    /// ```
    pub fn push(&mut self, value: T) -> usize {
        self.try_push(value)
            .expect("Should push while having space")
    }

    /// Removes the last element of the vector and returns it
    ///
    /// ```
    /// # use buffers::base_buffers::inline::InlineBuffer;
    /// # use generic_vec::Vector;
    /// # type ExampleBuffer = InlineBuffer<u32, 1>;
    /// let mut vec = Vector::<u32, ExampleBuffer>::new();
    /// vec.push(123);
    /// let value = vec.pop().expect("There is an element"); // value is 123
    /// # assert_eq!(value, 123);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            let value = unsafe { self.buffer.read_value(self.len) };
            Some(value)
        } else {
            None
        }
    }
}

impl<T, B> Vector<T, B>
where
    B: Buffer<Element = T> + Default,
{
    /// Creates a new vector by default-constructing the underlying buffer.
    pub fn new() -> Vector<T, B> {
        Self::from_buffer(Default::default())
    }
}

impl<T, B> Vector<T, B>
where
    B: Buffer<Element = T> + PtrBuffer,
{
    /// Returns an unsafe pointer to the start of the vector's buffer
    pub fn as_ptr(&self) -> *const T {
        unsafe { self.buffer.ptr(0) }
    }

    /// Returns an unsafe mutable pointer to the start of the vector's buffer
    pub fn as_mut_ptr(&mut self) -> *const T {
        unsafe { self.buffer.mut_ptr(0) }
    }
}

impl<T, B> Vector<T, B>
where
    B: Buffer<Element = T> + ContinuousMemoryBuffer,
{
    /// Extracts a slice containing the entire vector
    pub fn as_slice(&self) -> &[T] {
        unsafe { self.buffer.slice(0..self.len) }
    }

    /// Extracts a mutable slice containing the entire vector
    pub fn as_mut_slice(&self) -> &[T] {
        unsafe { self.buffer.slice(0..self.len) }
    }
}

impl<T, B> Default for Vector<T, B>
where
    B: Buffer<Element = T> + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, B: Buffer<Element = T>> Drop for Vector<T, B> {
    fn drop(&mut self) {
        // Safety: All the allocated elements are in 0 <= index < self.len.
        unsafe {
            self.buffer.manually_drop_range(0..self.len);
        }
    }
}

// SAFETY: The data is managed by the buffer. If it's Sync, so it's the vector.
unsafe impl<T, B: Buffer<Element = T> + Sync> Sync for Vector<T, B> {}

// SAFETY: The data is managed by the buffer. If it's Send, so it's the vector.
unsafe impl<T, B: Buffer<Element = T> + Send> Send for Vector<T, B> {}

#[cfg(test)]
mod tests {
    use buffers::base_buffers::{heap::HeapBuffer, inline::InlineBuffer};

    use super::*;

    type InlineVector = Vector<u32, InlineBuffer<u32, 4>>;

    #[test]
    fn pushed_values_should_increase_len() {
        let mut vec = InlineVector::new();
        assert_eq!(vec.len(), 0);

        vec.push(0);
        assert_eq!(vec.len(), 1);

        vec.push(1);
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn pushed_values_should_pop_in_reverse_order() {
        let mut vec = InlineVector::new();
        vec.push(123);
        vec.push(456);

        assert_eq!(vec.pop(), Some(456u32));
        assert_eq!(vec.pop(), Some(123u32));
    }

    #[test]
    fn drops_contents_on_drop() {
        use std::sync::atomic::{AtomicI64, Ordering};
        use test_utils::life_counter::LifeCounter;

        let counter = AtomicI64::new(0);
        {
            let mut vec = Vector::<LifeCounter, InlineBuffer<LifeCounter, 3>>::new();
            vec.push(LifeCounter::new(&counter));
            assert_eq!(counter.load(Ordering::SeqCst), 1);
        }
        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn should_increase_capacity_when_necessary() {
        let mut vec: Vector<u32, HeapBuffer<u32>> = Vector::new();

        vec.push(32);
        vec.push(32);

        assert!(vec.capacity() >= vec.len()); // This can probably be testes with a proptest
    }

    #[test]
    #[should_panic]
    fn should_panic_if_growing_is_not_allowed() {
        const SIZE: usize = 1;
        let mut vec: Vector<u32, InlineBuffer<u32, SIZE>> = Vector::new();
        for _ in 0..SIZE {
            vec.push(42);
        }

        assert_eq!(vec.capacity(), vec.len());

        vec.push(123);
    }
}
