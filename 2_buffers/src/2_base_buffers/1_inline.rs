use crate::interface::Buffer;
use std::mem::MaybeUninit;

pub struct InlineBuffer<T, const SIZE: usize> {
    array: [MaybeUninit<T>; SIZE],
}

impl<T, const SIZE: usize> Buffer<T> for InlineBuffer<T, SIZE> {
    fn capacity(&self) -> usize {
        SIZE
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn inline_buffer_should_have_the_capacity_of_its_size() {}
}
