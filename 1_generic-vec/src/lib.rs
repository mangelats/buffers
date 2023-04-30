use std::marker::PhantomData;

use buffers::{
    interface::{
        continuous_memory::ContinuousMemoryBuffer, ptrs::PtrBuffer, refs::RefBuffer,
        resize_error::ResizeError, Buffer,
    },
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
    /// # Example
    /// ```
    /// # use buffers::base_buffers::inline::InlineBuffer;
    /// # use generic_vec::Vector;
    /// let _vec = Vector::from_buffer(InlineBuffer::<u32, 1>::new());
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
    /// # Example
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
    ///
    /// # Example
    /// ```
    /// # use buffers::base_buffers::inline::InlineBuffer;
    /// # use generic_vec::Vector;
    /// let vec = Vector::<_, InlineBuffer::<u32, 150>>::new();
    /// assert_eq!(vec.capacity(), 150);
    /// ```
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    /// Reserves capacity for at least `additional` more elements to be inserted.
    /// It can request more memory in some cases, as this is meant to be optimized for
    /// conscutive inserts.
    ///
    /// Note that some buffers (like `InlineBuffer`) can't really grow.
    ///
    /// # Panics
    /// Panics if it cannot grow
    ///
    /// # Example
    /// ```
    /// # use generic_vec::Vector;
    /// let mut vec = Vector::<u32>::new();
    /// vec.reserve(150);
    /// assert!(vec.capacity() >= 150);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.try_reserve(additional)
            .expect("Couldn't reserve the necessary space")
    }

    /// Reserves capacity for at least `additional` more elements to be inserted.
    ///
    /// Note that unlike `reserve`, this will request exactly the additional size to the buffer.
    ///
    /// # Panics
    /// Panics if it cannot grow
    ///
    /// # Example
    /// ```
    /// # use generic_vec::Vector;
    /// let mut vec = Vector::<u32>::new();
    /// vec.reserve_exact(150);
    /// assert!(vec.capacity() >= 150);
    /// ```
    pub fn reserve_exact(&mut self, additional: usize) {
        self.try_reserve_exact(additional)
            .expect("Couldn't reserve the necessary space")
    }

    /// Tries reserves capacity for at least `additional` more elements to be inserted.
    ///
    /// Note that unlike `try_reserve`, this will request exactly the additional size to the buffer.
    ///
    /// # Examples
    /// Ok case:
    /// ```
    /// # use generic_vec::Vector;
    /// let mut vec = Vector::<u32>::new();
    /// let result = vec.try_reserve(150);
    /// assert_eq!(result.is_ok(), true);
    /// assert!(vec.capacity() >= 150);
    /// ```
    ///
    /// Failing case (an inline buffer cannot grow):
    /// ```
    /// # use buffers::base_buffers::inline::InlineBuffer;
    /// # use generic_vec::Vector;
    /// let mut vec = Vector::<u32, InlineBuffer<_, 10>>::new();
    /// let result = vec.try_reserve(150);
    /// assert_eq!(result.is_err(), true);
    /// ```
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), ResizeError> {
        // TODO Grow exponentially
        self.try_reserve_exact(additional)
    }

    /// Tries reserves capacity for at least `additional` more elements to be inserted.
    ///
    /// Note that unlike `try_reserve`, this will request exactly the additional size to the buffer.
    ///
    /// # Examples
    /// Ok case:
    /// ```
    /// # use generic_vec::Vector;
    /// let mut vec = Vector::<u32>::new();
    /// let result = vec.try_reserve_exact(150);
    /// assert_eq!(result.is_ok(), true);
    /// assert!(vec.capacity() >= 150);
    /// ```
    ///
    /// Failing case (an inline buffer cannot grow):
    /// ```
    /// # use buffers::base_buffers::inline::InlineBuffer;
    /// # use generic_vec::Vector;
    /// let mut vec = Vector::<u32, InlineBuffer<_, 10>>::new();
    /// let result = vec.try_reserve_exact(150);
    /// assert_eq!(result.is_err(), true);
    /// ```
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), ResizeError> {
        let target = self.len() + additional;
        if target > self.capacity() {
            // SAFETY: It's bigger than the current size
            unsafe { self.buffer.try_grow(target) }
        } else {
            Ok(())
        }
    }

    /// Shrinks the capacity of the vector as much as possible.
    ///
    /// # Example
    /// ```
    /// # use buffers::base_buffers::heap::HeapBuffer;
    /// # use generic_vec::Vector;
    /// let mut vec = Vector::<u32, HeapBuffer<_>>::new();
    /// vec.reserve(10);
    /// assert!(vec.capacity() >= 10);
    ///
    /// vec.shrink_to_fit();
    /// assert_eq!(vec.capacity(), 0);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.shrink_to(self.len())
    }

    /// Hints the vector that it may shrink up to a lower bound.
    ///
    /// The capacity will remain at least as large as both the length and the supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Example
    /// ```
    /// # use buffers::base_buffers::heap::HeapBuffer;
    /// # use generic_vec::Vector;
    /// let mut vec = Vector::<u32, HeapBuffer<_>>::new();
    /// vec.reserve(10);
    /// assert!(vec.capacity() >= 10);
    ///
    /// vec.shrink_to(0);
    /// assert_eq!(vec.capacity(), 0);
    /// ```
    pub fn shrink_to(&mut self, min_capacity: usize) {
        let target = std::cmp::max(min_capacity, self.len());
        if target < self.capacity() {
            // SAFETY: it should get OOM but the buffer may not be able to shrink (eg. InlineBuffer)
            // this still is considered successful in that case
            let _ = unsafe { self.buffer.try_shrink(min_capacity) };
        }
    }

    /// Shortens the vector, keeping the first len elements and dropping the rest.
    ///
    /// If len is greater than the vector’s current length, this has no effect.
    ///
    /// Note that this method has no effect on the allocated capacity of the vector.
    pub fn truncate(&mut self, keep_n_first: usize) {
        if keep_n_first < self.len {
            // SAFETY: the values from keep to len exist
            unsafe {
                self.buffer.manually_drop_range(keep_n_first..self.len);
            }
            self.len = keep_n_first
        }
    }
    /// Removes an element from the vector and returns it.
    ///
    /// The removed element is replaced by the last element of the vector.
    ///
    /// This does not preserve ordering, but is O(1). If you need to preserve the element order, use remove instead.
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds.
    ///
    /// # Example
    /// ```
    /// # use buffers::base_buffers::heap::HeapBuffer;
    /// # use generic_vec::Vector;
    /// let mut vec = Vector::<u32, HeapBuffer<_>>::new();
    /// vec.reserve(4);
    /// vec.push(0);
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    ///
    /// vec.swap_remove(1);
    /// ```
    pub fn swap_remove(&mut self, index: usize) -> T {
        if index >= self.len {
            panic!("Index out of bounds")
        }
        self.len -= 1;

        // SAFETY: index is in bounds
        let current = unsafe { self.buffer.read_value(index) };

        // Move only when necessary
        if self.len != index {
            unsafe {
                let value = self.buffer.read_value(self.len);
                self.buffer.write_value(index, value);
            }
        }

        current
    }

    /// Inserts an element at position `index` within the vector, shifting all elements after it to the right.
    ///
    /// #Panics
    ///     
    /// Panics if `index > len`.
    pub fn insert(&mut self, index: usize, element: T) {
        if index > self.len {
            panic!("Index out of bounds")
        }

        if self.len >= self.buffer.capacity() {
            let resize_result = unsafe { self.buffer.try_grow(self.next_size()) };
            resize_result.expect("Cannot grow the buffer when trying to insert a new value")
        }

        unsafe {
            self.buffer.shift_right(index..self.len, 1);
            self.buffer.write_value(index, element);
        }
        self.len += 1;
    }

    /// Removes and returns the element at position `index` within the vector,
    /// shifting all elements after it to the left.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.len {
            panic!("Index out of bounds")
        }

        let result = unsafe { self.buffer.read_value(index) };
        unsafe {
            self.buffer.shift_left(index..self.len, 1);
        }
        self.len -= 1;
        result
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
                self.buffer.try_grow(self.next_size()).map_err(|_| ())?;
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
            // SAFETY: self.len-1 is the last element, which we will pop
            self.len -= 1;
            let value = unsafe { self.buffer.read_value(self.len) };
            Some(value)
        } else {
            None
        }
    }

    fn next_size(&self) -> usize {
        self.len + 1
    }
}

impl<T, B> Vector<T, B>
where
    B: Buffer<Element = T> + Default,
{
    /// Creates a new vector by default-constructing the underlying buffer.
    ///
    /// # Example
    ///
    /// ```
    /// # use generic_vec::Vector;
    /// let _vec = Vector::<u32>::new();
    /// ```
    pub fn new() -> Vector<T, B> {
        Self::from_buffer(Default::default())
    }
}

impl<T, B> Vector<T, B>
where
    B: Buffer<Element = T> + PtrBuffer,
{
    /// Returns an unsafe pointer to the start of the vector's buffer
    pub fn as_ptr(&self) -> B::ConstantPointer {
        // SAFETY: even if empty, the (unsafe) pointer is corrent
        unsafe { self.buffer.ptr(0) }
    }

    /// Returns an unsafe mutable pointer to the start of the vector's buffer
    pub fn as_mut_ptr(&mut self) -> B::MutablePointer {
        // SAFETY: even if empty, the (unsafe) pointer is corrent
        unsafe { self.buffer.mut_ptr(0) }
    }
}

impl<T, B> Vector<T, B>
where
    B: Buffer<Element = T> + RefBuffer,
{
    /// Get a reference to the element in index
    ///
    /// # Safety
    /// index < self.len()
    pub fn index(&self, index: usize) -> B::ConstantReference<'_> {
        debug_assert!(index < self.len());
        // SAFETY: values up to len exist
        unsafe { self.buffer.index(index) }
    }

    /// Get a mutable reference to the element in index
    ///
    /// # Safety
    /// index < self.len()
    pub fn mut_index(&mut self, index: usize) -> B::MutableReference<'_> {
        debug_assert!(index < self.len());
        // SAFETY: values up to len exist
        unsafe { self.buffer.mut_index(index) }
    }
}

impl<T, B> Vector<T, B>
where
    B: Buffer<Element = T> + ContinuousMemoryBuffer,
{
    /// Extracts a slice containing the entire vector
    pub fn as_slice(&self) -> &[T] {
        // SAFETY: values up to len exist
        unsafe { self.buffer.slice(0..self.len) }
    }

    /// Extracts a mutable slice containing the entire vector
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: values up to len exist
        unsafe { self.buffer.mut_slice(0..self.len) }
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

    #[test]
    fn should_be_able_to_get_a_reference() {
        const SIZE: usize = 10;
        let mut vec: Vector<u32, InlineBuffer<u32, SIZE>> = Vector::new();
        for i in 0..SIZE {
            vec.push(i.try_into().unwrap());
        }

        assert_eq!(*vec.index(3), 3);
    }

    #[test]
    fn should_be_able_to_get_a_mutable_reference() {
        const SIZE: usize = 10;
        let mut vec: Vector<u32, InlineBuffer<u32, SIZE>> = Vector::new();
        for i in 0..SIZE {
            vec.push(i.try_into().unwrap());
        }

        assert_eq!(*vec.index(3), 3);
        *vec.mut_index(3) = 4;
        assert_eq!(*vec.index(3), 4);
    }
}
