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
    }
}
