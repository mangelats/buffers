use crate::interface::{
    continuous_memory::ContinuousMemoryBuffer, ptrs::PtrBuffer, refs::DefaultRefBuffer, Buffer,
};
use std::mem::MaybeUninit;

/// Buffer based on a fixed-sized array, so it cannot grow or shrink.
///
/// This means that the memory is contiguous and it can be used in the stack because the size is known at compile time.
/// It can be used a building block for some other more suffisticated buffers.
pub struct InlineBuffer<T, const SIZE: usize> {
    array: [MaybeUninit<T>; SIZE],
}

impl<T, const SIZE: usize> InlineBuffer<T, SIZE> {
    /// Create a new, empty inline buffer
    pub fn new() -> Self {
        InlineBuffer {
            array: MaybeUninit::uninit_array(),
        }
    }

    /// Get a contant reference maybe-uninit value in the specified index.
    ///
    /// # SAFETY
    /// `index` needs to be in bounds (`0 <= index < SIZE`). It's undefined behaviour when not.
    fn at(&self, index: usize) -> &MaybeUninit<T> {
        debug_assert!(index < SIZE);
        &self.array[index]
    }

    /// Get a mutable reference maybe-uninit value in the specified index.
    ///
    /// ## SAFETY
    /// `index` needs to be in bounds (`0 <= index < SIZE`). It's undefined behaviour when not.
    fn mut_at(&mut self, index: usize) -> &mut MaybeUninit<T> {
        debug_assert!(index < SIZE);
        &mut self.array[index]
    }
}

impl<T, const SIZE: usize> Buffer for InlineBuffer<T, SIZE> {
    type Element = T;

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

impl<T, const SIZE: usize> PtrBuffer for InlineBuffer<T, SIZE> {
    type ConstantPointer = *const T;
    type MutablePointer = *mut T;

    unsafe fn ptr(&self, index: usize) -> *const T {
        debug_assert!(index < SIZE);
        self.at(index).as_ptr()
    }

    unsafe fn mut_ptr(&mut self, index: usize) -> *mut T {
        debug_assert!(index < SIZE);
        self.mut_at(index).as_mut_ptr()
    }
}

impl<T, const SIZE: usize> ContinuousMemoryBuffer for InlineBuffer<T, SIZE> {}
impl<T, const SIZE: usize> DefaultRefBuffer for InlineBuffer<T, SIZE> {}

impl<T, const SIZE: usize> Default for InlineBuffer<T, SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

// SAFETY: Because the data is inlined, Sync is safe when the underlying type is.
unsafe impl<T: Sync, const SIZE: usize> Sync for InlineBuffer<T, SIZE> {}

// SAFETY: Because the data is inlined, Send is safe when the underlying type is.
unsafe impl<T: Send, const SIZE: usize> Send for InlineBuffer<T, SIZE> {}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicI64, Ordering};

    use test_utils::life_counter::LifeCounter;

    use super::*;

    #[test]
    fn inline_buffer_should_have_the_capacity_of_its_size() {
        let vec = InlineBuffer::<u32, 123>::new();
        assert_eq!(vec.capacity(), 123);
    }

    #[test]
    fn inline_buffer_should_be_defaultable() {
        let _: InlineBuffer<u32, 123> = Default::default();
    }

    #[test]
    fn inline_buffer_should_can_read_previously_written_values() {
        let mut vec = InlineBuffer::<u32, 123>::new();
        for x in 1..3 {
            unsafe { vec.write_value(0, x) };
            let r = unsafe { vec.read_value(0) };

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

    #[test]
    fn manually_drop_should_call_destructor() {
        let counter = AtomicI64::new(0);
        let mut buffer = InlineBuffer::<LifeCounter<'_>, 1>::new();

        unsafe { buffer.write_value(0, LifeCounter::new(&counter)) };
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        unsafe { buffer.manually_drop(0) };
        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }
}
