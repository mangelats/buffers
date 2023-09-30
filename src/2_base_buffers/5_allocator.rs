use std::{
    alloc::{Allocator, Global, Layout},
    marker::PhantomData,
    ptr::NonNull,
};

use crate::interface::{
    contiguous_memory::ContiguousMemoryBuffer, copy_value::CopyValueBuffer, ptrs::PtrBuffer,
    refs::RefBuffer, resize_error::ResizeError, Buffer,
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

    unsafe fn read(&self, index: usize) -> T {
        // SAFETY: [`Buffer::take`] ensures that the position is valid and
        // filled.
        let ptr = unsafe { self.ptr(index) };
        // SAFETY: `self.ptr` ensures that the pointer is valid.
        // [`Buffer::take`] ensures that the position is filled.
        unsafe { std::ptr::read(ptr) }
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

    unsafe fn take(&mut self, index: usize) -> T {
        // SAFETY: it has the same requirements
        unsafe { self.read(index) }
    }

    unsafe fn put(&mut self, index: usize, value: T) {
        // SAFETY: [`Buffer::put`] ensures that the position is valid and empty.
        let ptr = unsafe { self.mut_ptr(index) };
        // SAFETY: [`PtrBuffer::mut_ptr`] ensures that the pointer is valid.
        // [`Buffer::put`] ensures that the position is empty.
        unsafe { std::ptr::write(ptr, value) };
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        // SAFETY: [`Buffer::manually_drop`] ensures that the position is valid
        // and filled.
        let ptr = unsafe { self.mut_ptr(index) };
        // SAFETY: [`PtrBuffer::mut_ptr`] ensures that the pointer is valid.
        // [`Buffer::manually_drop`] ensures that the position is filled.
        unsafe { std::ptr::drop_in_place(ptr) };
    }

    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        let ptr = if self.cap > 0 {
            // SAFETY: `self.cap` is checked in the conditional.
            // [`Buffer::try_grow`] ensures that `target` > `self.cap` (which is
            // 0)
            unsafe { try_grow(&self.alloc, self.ptr, self.cap, target) }
        } else {
            // SAFETY: `self.cap` is checked to be grater than 0, which means
            // that `self.buffer_start` is not dangling.
            // [`Buffer::try_grow`] ensures that `target` > `self.cap` (which
            // implies `target` != `self.cap`)
            unsafe { try_allocate(&self.alloc, target) }
        }?;
        self.update_buffer(ptr, target);
        Ok(())
    }

    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError> {
        if target == 0 {
            // SAFETY: [`Buffer::try_shrink`] ensures `target` < `self.cap`.
            // This means that `self.cap` > 0 (conditional) and thus
            // `self.buffer_start` is not dangling.
            unsafe { try_deallocate(&self.alloc, self.ptr, self.cap)? };
            self.update_buffer(NonNull::dangling(), 0);
            Ok(())
        } else {
            // SAFETY: `target` is not 0 and it only allows positive values,
            // thus `target` > 0 at this point.
            // [`Buffer::try_shrink`] ensures `target` < `self.cap`. This means
            // that `target` != `self.cap`. Also `self.cap` > 0 (conditional)
            // and thus `self.buffer_start` is not dangling.
            let ptr = unsafe { try_shrink(&self.alloc, self.ptr, self.cap, target)? };
            self.update_buffer(ptr, target);
            Ok(())
        }
    }
}

impl<T: Copy, A: Allocator> CopyValueBuffer for AllocatorBuffer<T, A> {
    unsafe fn copy(&self, index: usize) -> T {
        // SAFETY: it has the same requirements
        unsafe { self.read(index) }
    }
}

impl<T, A: Allocator> PtrBuffer for AllocatorBuffer<T, A> {
    type ConstantPointer = *const T;
    type MutablePointer = *mut T;

    unsafe fn ptr(&self, index: usize) -> *const Self::Element {
        let ptr = self.ptr.as_ptr();

        // SAFETY: `ptr` is at the start, `ptr.add(index)` points to the array's
        // position. [`PtrBuffer::ptr`] requires that the index is valid and
        // filled. Thus the pointer also is.
        unsafe { ptr.add(index) }
    }

    unsafe fn mut_ptr(&mut self, index: usize) -> *mut Self::Element {
        let ptr = self.ptr.as_ptr();

        // SAFETY: `ptr` is at the start, `ptr.add(index)` points to the array's
        // position. [`PtrBuffer::mut_ptr`] requires that the index is valid and
        // filled. Thus the pointer also is.
        unsafe { ptr.add(index) }
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
        // SAFETY: [`RefBuffer::index`] has at least the same requirements as
        // [`PtrBuffer::ptr`].
        let ptr = unsafe { self.ptr(index) };
        // SAFETY: [`PtrBuffer::ptr`] requires that the pointer can be
        // dereferenced.
        unsafe { &*ptr }
    }

    unsafe fn mut_index<'a: 'b, 'b>(&'a mut self, index: usize) -> &'b mut T {
        // SAFETY: [`RefBuffer::mut_index`] has at least the same requirements
        // as [`PtrBuffer::mut_ptr`].
        let ptr = unsafe { self.mut_ptr(index) };
        // SAFETY: [`PtrBuffer::mut_ptr`] requires that the pointer can be
        // dereferenced.
        unsafe { &mut *ptr }
    }
}

impl<T, A: Allocator> ContiguousMemoryBuffer for AllocatorBuffer<T, A> {}

impl<T, A: Allocator + Default> Default for AllocatorBuffer<T, A> {
    fn default() -> Self {
        Self::new()
    }
}

// SAFETY: As a buffer it's not its responsabilities to clean the values that it
// saves. The container should use [`Buffer::manually_drop`] and
// [`Buffer::manually_drop_range`] to properly drop the values it contains.
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
///   * `old_ptr` must not be null or dangling.
///   * `old_ptr` must be managed by `alloc`.
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

    // SAFETY:
    //  * `old_ptr` should be currently managed by `alloc` (precondition).
    //  * `old_layout` is recreated for the exact block of memory.
    //  * Since `old_size` < `new_size`, then `old_layout.size()` <
    //    `new_layout.size()`.
    let new_ptr = unsafe { alloc.grow(old_ptr.cast(), old_layout, new_layout)? };

    Ok(new_ptr.cast())
}

/// Internal utility function that tries to shrink a an array of a given size
/// using the provided allocator.
///
/// # Safety
///   * `alloc` must be able to handle `T`.
///   * `old_ptr` must not be null or dangling.
///   * `old_ptr` must be managed by `alloc`.
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

    // SAFETY:
    //  * `old_ptr` should be currently managed by `alloc` (precondition).
    //  * `old_layout` is recreated for the exact block of memory.
    //  * Since `old_size` > `new_size`, then `old_layout.size()` >
    //    `new_layout.size()`.
    let new_ptr = unsafe { alloc.shrink(old_ptr.cast(), old_layout, new_layout)? };

    Ok(new_ptr.cast())
}

/// Internal utility function that tries to deallocate an array using an
/// allocator.
///
/// # Safety
///   * `alloc` must be able to handle `T`.
///   * `old_ptr` must not be null or dangling.
///   * `old_ptr` must be managed by `alloc`.
///   * `old_size` must be the size returned by the size of the array.
unsafe fn try_deallocate<T, A: Allocator>(
    alloc: &A,
    old_ptr: NonNull<T>,
    old_size: usize,
) -> Result<(), ResizeError> {
    let old_layout = Layout::array::<T>(old_size)?;

    // SAFETY:
    //  * `old_ptr` should be currently managed by `alloc` (precondition).
    //  * `old_layout` is recreated for the exact block of memory.
    unsafe { alloc.deallocate(old_ptr.cast(), old_layout) };
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
