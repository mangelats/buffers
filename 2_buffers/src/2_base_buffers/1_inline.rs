use crate::interface::Buffer;
use std::mem::MaybeUninit;

pub struct InlineBuffer<T, const SIZE: usize> {
    array: [MaybeUninit<T>; SIZE],
}

impl<T, const SIZE: usize> InlineBuffer<T, SIZE> {
    pub fn new() -> Self {
        InlineBuffer {
            array: MaybeUninit::uninit_array(),
        }
    }

    pub fn ptr(&self, index: usize) -> *const T {
        self.array[index].as_ptr()
    }

    pub fn mut_ptr(&mut self, index: usize) -> *mut T {
        self.array[index].as_mut_ptr()
    }
}

impl<T, const SIZE: usize> Buffer<T> for InlineBuffer<T, SIZE> {
    fn capacity(&self) -> usize {
        SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_buffer_should_have_the_capacity_of_its_size() {
        let vec = InlineBuffer::<u32, 123>::new();
        assert_eq!(vec.capacity(), 123);
    }

    #[test]
    fn inline_buffer_should_can_read_previously_written_values() {
        let mut vec = InlineBuffer::<u32, 123>::new();
        for x in 1..3 {}
    }
}
