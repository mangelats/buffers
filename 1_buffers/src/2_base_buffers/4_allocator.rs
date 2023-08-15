use std::{
    alloc::{Allocator, Global, Layout},
    marker::PhantomData,
    ptr::NonNull,
};

use crate::interface::{
    contiguous_memory::ContiguousMemoryBuffer, ptrs::PtrBuffer, refs::RefBuffer,
    resize_error::ResizeError, Buffer,
};

/// Buffer that dynamically allocates using an [`Allocator`].
///
/// Using the [`Global`] allocator (which is done by default) should be
/// equivalent to using [`super::heap::HeapBuffer`].
///
/// It requires the `allocator` feature.
pub struct AllocatorBuffer<T, A: Allocator = Global> {
    ptr: NonNull<T>,
    cap: usize,
    alloc: A,
    _marker: PhantomData<T>,
}

impl<T, A: Allocator + Default> AllocatorBuffer<T, A> {
    /// Makes an empty buffer by default-constructing the allocator.
    pub fn new() -> Self {
        Self::with_allocator(Default::default())
    }
}

impl<T, A: Allocator> AllocatorBuffer<T, A> {
    /// Make an empty buffer given an allocator.
    pub fn with_allocator(alloc: A) -> Self {
        Self {
            ptr: NonNull::dangling(),
            cap: 0,
            alloc,
            _marker: PhantomData,
        }
    }

    /// Internal function that sets the capacity and raw buffer pointer
    fn update_buffer(&mut self, ptr: NonNull<T>, cap: usize) {
        self.cap = cap;
        self.ptr = ptr;
    }
}

impl<T, A: Allocator> Buffer for AllocatorBuffer<T, A> {
    type Element = T;

    fn capacity(&self) -> usize {
        self.cap
    }

    unsafe fn read_value(&self, index: usize) -> T {
        std::ptr::read(self.ptr(index))
    }

    unsafe fn write_value(&mut self, index: usize, value: T) {
        std::ptr::write(self.mut_ptr(index), value)
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        std::ptr::drop_in_place(self.mut_ptr(index));
    }

    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        let ptr = if self.cap > 0 {
            try_grow(&self.alloc, self.ptr, self.cap, target)
        } else {
            try_allocate(&self.alloc, target)
        }?;
        self.update_buffer(ptr, target);
        Ok(())
    }

    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError> {
        if target == 0 {
            try_deallocate(&self.alloc, self.ptr, self.cap)?;
            self.update_buffer(NonNull::dangling(), 0);
            Ok(())
        } else {
            let ptr = try_shrink(&self.alloc, self.ptr, self.cap, target)?;
            self.update_buffer(ptr, target);
            Ok(())
        }
    }
}

impl<T, A: Allocator> PtrBuffer for AllocatorBuffer<T, A> {
    type ConstantPointer = *const T;
    type MutablePointer = *mut T;

    unsafe fn ptr(&self, index: usize) -> *const Self::Element {
        self.ptr.as_ptr().add(index)
    }

    unsafe fn mut_ptr(&mut self, index: usize) -> *mut Self::Element {
        self.ptr.as_ptr().add(index)
    }
}

impl<T, A: Allocator> RefBuffer for AllocatorBuffer<T, A> {
    type ConstantReference<'a> = &'a T
    where
        Self: 'a;
    type MutableReference<'a> = &'a mut T
    where
        Self: 'a;

    unsafe fn index<'a: 'b, 'b>(&'a self, index: usize) -> &'b T {
        &*self.ptr(index)
    }

    unsafe fn mut_index<'a: 'b, 'b>(&'a mut self, index: usize) -> &'b mut T {
        &mut *self.mut_ptr(index)
    }
}

impl<T, A: Allocator> ContiguousMemoryBuffer for AllocatorBuffer<T, A> {}

impl<T, A: Allocator + Default> Default for AllocatorBuffer<T, A> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<#[may_dangle] T, A: Allocator> Drop for AllocatorBuffer<T, A> {
    fn drop(&mut self) {
        if self.cap != 0 {
            // SAFETY: At this point all content should have been dropped
            unsafe {
                // Even if it fails, we can only ignore the error
                let _ = try_deallocate(&self.alloc, self.ptr, self.cap);
            }
        }
    }
}

/// Internal utility function that tries to allocate a new array of a given size
/// using the provided allocator.
///
/// # Safety
///   * `alloc` must be able to handle `T`.
///   * `size` must be bigger than zero.
unsafe fn try_allocate<T, A: Allocator>(alloc: &A, size: usize) -> Result<NonNull<T>, ResizeError> {
    debug_assert!(size > 0);
    let new_layout = Layout::array::<T>(size)?;

    let new_ptr = alloc.allocate(new_layout)?;

    Ok(new_ptr.cast())
}

/// Internal utility function that tries to grow a an array of a given size
/// using the provided allocator.
///
/// # Safety
///   * `alloc` must be able to handle `T`.
///   * `old_ptr` must be valid (must not be dangling).
///   * `old_size` must be the size returned by the size of the array.
///   * `new_size` must be biggen than `old_size` and zero.
unsafe fn try_grow<T, A: Allocator>(
    alloc: &A,
    old_ptr: NonNull<T>,
    old_size: usize,
    new_size: usize,
) -> Result<NonNull<T>, ResizeError> {
    debug_assert!(new_size > old_size);

    let old_layout = Layout::array::<T>(old_size)?;
    let new_layout = Layout::array::<T>(new_size)?;

    let new_ptr = alloc.grow(old_ptr.cast(), old_layout, new_layout)?;

    Ok(new_ptr.cast())
}

/// Internal utility function that tries to shrink a an array of a given size
/// using the provided allocator.
///
/// # Safety
///   * `alloc` must be able to handle `T`.
///   * `old_ptr` must be valid (must not be dangling).
///   * `old_size` must be the size returned by the size of the array.
///   * `new_size` must be biggen than  zero.
///   * `new_size` must be smaller than `old_size`.
unsafe fn try_shrink<T, A: Allocator>(
    alloc: &A,
    old_ptr: NonNull<T>,
    old_size: usize,
    new_size: usize,
) -> Result<NonNull<T>, ResizeError> {
    debug_assert!(new_size > 0);
    debug_assert!(new_size < old_size);

    let old_layout = Layout::array::<T>(old_size)?;
    let new_layout = Layout::array::<T>(new_size)?;

    let new_ptr = alloc.shrink(old_ptr.cast(), old_layout, new_layout)?;

    Ok(new_ptr.cast())
}

/// Internal utility function that tries to deallocate an array using an
/// allocator.
///
/// # Safety
///   * `alloc` must be able to handle `T`.
///   * `old_ptr` must be valid (must not be dangling).
///   * `old_size` must be the size returned by the size of the array.
unsafe fn try_deallocate<T, A: Allocator>(
    alloc: &A,
    old_ptr: NonNull<T>,
    old_size: usize,
) -> Result<(), ResizeError> {
    let old_layout = Layout::array::<T>(old_size)?;
    alloc.deallocate(old_ptr.cast(), old_layout);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_grow_from_default() {
        const TARGET: usize = 1;

        let mut buffer = AllocatorBuffer::<i32, Global>::new();

        // SAFETY: 0 < TARGET
        unsafe {
            buffer.try_grow(TARGET).unwrap();
        }

        assert!(buffer.capacity() >= TARGET);
    }

    #[test]
    fn can_grow_twice() {
        const TARGET1: usize = 1;
        const TARGET2: usize = 10;

        let mut buffer = AllocatorBuffer::<i32, Global>::new();

        // SAFETY: 0 < TARGET1 < TARGET2
        unsafe {
            buffer.try_grow(TARGET1).unwrap();
            buffer.try_grow(TARGET2).unwrap();
        }

        assert!(buffer.capacity() >= TARGET2);
    }

    #[test]
    fn can_shrink() {
        const TARGET1: usize = 64;
        const TARGET2: usize = 1;

        let mut buffer = AllocatorBuffer::<i32, Global>::new();

        // SAFETY: 0 < TARGET2 < TARGET1
        unsafe {
            buffer.try_grow(TARGET1).unwrap();
            buffer.try_shrink(TARGET2).unwrap();
        }

        assert!(buffer.capacity() < TARGET1);
        assert!(buffer.capacity() >= TARGET2);
    }

    #[test]
    fn can_shrink_to_nothing() {
        const TARGET1: usize = 64;
        const TARGET2: usize = 0;

        let mut buffer = AllocatorBuffer::<i32, Global>::new();

        // SAFETY: 0 == TARGET2 < TARGET1
        unsafe {
            buffer.try_grow(TARGET1).unwrap();
            buffer.try_shrink(TARGET2).unwrap();
        }

        assert!(buffer.capacity() < TARGET1);
        assert!(buffer.capacity() == TARGET2);
    }
}
