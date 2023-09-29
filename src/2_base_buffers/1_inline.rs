use crate::interface::{
    contiguous_memory::ContiguousMemoryBuffer, ptrs::PtrBuffer, refs::RefBuffer,
    resize_error::ResizeError, Buffer,
};
use std::mem::MaybeUninit;

/// Buffer based on an inline fixed-sized array. It cannot grow or shrink. This
/// also means that the memory is contiguous and it can be used in the stack
/// because the size is known at compile time.
///
/// It can also be combined with [`std::boxed::Box`] to move the array on the
/// heap (since `Box<AnyBuffer>` is also a buffer).
pub struct InlineBuffer<T, const SIZE: usize> {
    array: [MaybeUninit<T>; SIZE],
}

impl<T, const SIZE: usize> InlineBuffer<T, SIZE> {
    /// Create a new empty inline buffer.
    pub fn new() -> Self {
        InlineBuffer {
            array: MaybeUninit::uninit_array(),
        }
    }

    /// Get a constant reference to an element in the specified `index` that may
    /// or may not be initialized.
    ///
    /// # SAFETY
    ///   * `index` must be valid.
    fn at(&self, index: usize) -> &MaybeUninit<T> {
        debug_assert!(index < SIZE);
        &self.array[index]
    }

    /// Get a mutable reference to an element in the specified `index` that may
    /// or may not be initialized.
    ///
    /// # SAFETY
    ///   * `index` must be valid.
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

    unsafe fn read_value(&mut self, index: usize) -> T {
        // SAFETY: `index` is unsafe with requirements that ensures that
        // [`PtrBuffer::ptr`] can be used.
        let ptr = unsafe { self.ptr(index) };
        // SAFETY: if `index` is a valid position, `ptr` is valid to read from.
        unsafe { ptr.read() }
    }

    unsafe fn write_value(&mut self, index: usize, value: T) {
        // SAFETY: `index` is unsafe with requirements that ensures that
        // [`PtrBuffer::ptr`] can be used.
        let ptr = unsafe { self.mut_ptr(index) };
        // SAFETY: if `index` is an empty position, `ptr` is valid to write to.
        unsafe { ptr.write(value) }
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        // SAFETY: `index` is unsafe with requirements that ensures that
        // [`PtrBuffer::ptr`] can be used.
        let ptr = unsafe { self.mut_ptr(index) };
        // SAFETY: if `index` is a valid position, `ptr` is valid to drop.
        unsafe { std::ptr::drop_in_place(ptr) };
    }

    unsafe fn try_grow(&mut self, _target: usize) -> Result<(), ResizeError> {
        Err(ResizeError::UnsupportedOperation)
    }

    unsafe fn try_shrink(&mut self, _target: usize) -> Result<(), ResizeError> {
        Err(ResizeError::UnsupportedOperation)
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

impl<T, const SIZE: usize> RefBuffer for InlineBuffer<T, SIZE> {
    type ConstantReference<'a> = &'a T
    where
        Self: 'a;
    type MutableReference<'a> = &'a mut T
    where
        Self: 'a;

    unsafe fn index<'a: 'b, 'b>(&'a self, index: usize) -> &'b T {
        // SAFETY: `index` is unsafe with requirements that ensures that
        // [`PtrBuffer::ptr`] can be used.
        let ptr = unsafe { self.ptr(index) };
        // SAFETY: [`PtrBuffer::ptr`] ensures that the pointer can be
        // derefferenced.
        unsafe { &*ptr }
    }

    unsafe fn mut_index<'a: 'b, 'b>(&'a mut self, index: usize) -> &'b mut T {
        // SAFETY: `mut_index` is unsafe with requirements that ensures that
        // [`PtrBuffer::mut_ptr`] can be used.
        let ptr = unsafe { self.mut_ptr(index) };
        // SAFETY: [`PtrBuffer::mut_ptr`] ensures that the pointer can be
        // derefferenced.
        unsafe { &mut *ptr }
    }
}

impl<T, const SIZE: usize> ContiguousMemoryBuffer for InlineBuffer<T, SIZE> {}

impl<T, const SIZE: usize> Default for InlineBuffer<T, SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicI64, Ordering};

    use crate::test_utils::life_counter::LifeCounter;

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
