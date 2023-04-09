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

    pub unsafe fn ptr(&self, index: usize) -> *const T {
        self.array[index].as_ptr()
    }

    pub unsafe fn mut_ptr(&mut self, index: usize) -> *mut T {
        self.array[index].as_mut_ptr()
    }
}

impl<T, const SIZE: usize> Buffer<T> for InlineBuffer<T, SIZE> {
    fn capacity(&self) -> usize {
        SIZE
    }

    unsafe fn read_value(&self, index: usize) -> T {
        self.ptr(index).read()
    }

    unsafe fn write_value(&mut self, index: usize, value: T) {
        self.mut_ptr(index).write(value)
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        std::ptr::drop_in_place(self.mut_ptr(index));
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
        for x in 1..3 {
            unsafe { vec.mut_ptr(0).write(x) };
            let r = unsafe { vec.ptr(0).read() };

            assert_eq!(x, r)
        }
    }

    #[test]
    fn inline_buffer_should_be_able_to_read_multiple_values() {
        let mut vec = InlineBuffer::<usize, 123>::new();
        for x in 1..3 {
            unsafe { vec.write_value(x, x * 2) };
        }
        for x in 1..3 {
            let r = unsafe { vec.read_value(x) };
            assert_eq!(r, x * 2)
        }
    }
}
