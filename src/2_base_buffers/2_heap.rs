use std::{
    alloc::Layout,
    marker::PhantomData,
    ptr::{self, NonNull},
};

use crate::interface::{
    contiguous_memory::ContiguousMemoryBuffer, copy_value::CopyValueBuffer, ptrs::PtrBuffer,
    refs::RefBuffer, resize_error::ResizeError, Buffer,
};

/// Buffer implementation using a heap-allocated contiguous array.
///
/// This implementation uses the allocation functions on [`std::alloc`].
pub struct HeapBuffer<T> {
    buffer_start: NonNull<T>,
    cap: usize,
    _marker: PhantomData<T>,
}

impl<T> HeapBuffer<T> {
    /// Makes a new default-sized `HeapBuffer`.
    ///
    /// ```
    /// # use buffers::base_buffers::heap::HeapBuffer;
    /// let buffer = HeapBuffer::<u32>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            buffer_start: NonNull::dangling(),
            cap: 0,
            _marker: PhantomData,
        }
    }

    /// Internal utility that reads `index`. Used both for copying and for
    /// extracting the value.
    ///
    /// # Safety
    ///   * `index` must be less than `capacity`.
    ///   * The `index` position must be filled.
    unsafe fn read(&self, index: usize) -> T {
        // SAFETY: `index` is unsafe with requirements that ensures that
        // [`PtrBuffer::ptr`] can be used.
        let ptr = unsafe { self.ptr(index) };
        // SAFETY: if `index` is a valid position, `ptr` is valid to read from.
        unsafe { ptr.read() }
    }

    /// Internal function that allocates a new array into the heap.
    ///
    /// # Safety
    ///   * `self.cap` must be 0.
    ///   * `target` must be greater than 0.
    unsafe fn allocate_array(&mut self, target: usize) -> Result<(), ResizeError> {
        debug_assert!(self.cap == 0);
        debug_assert!(target > 0);

        // SAFETY: This requirement is propegated to this function docs.
        let ptr = unsafe { try_array_alloc(target)? };
        self.update_buffer(ptr, target);
        Ok(())
    }

    /// Internal function that tries to resize the array.
    ///
    /// # Safety
    ///   * `self.buffer_start` cannot be dangling.
    ///   * `target` must be greater than zero.
    ///   * `target` must be different than `self.cap`.
    unsafe fn resize_array(&mut self, target: usize) -> Result<(), ResizeError> {
        debug_assert!(target > 0);
        debug_assert!(target != self.cap);
        // SAFETY: Requirements propegated into this function ones
        let ptr = unsafe { try_array_realloc(self.buffer_start, self.cap, target)? };
        self.update_buffer(ptr, target);
        Ok(())
    }

    /// Internal function that deallocates the array.
    ///
    /// # Safety
    ///   * `self.buffer_start` cannot be dangling.
    ///   * `self.cap` must be greater than zero.
    unsafe fn deallocate_array(&mut self) -> Result<(), ResizeError> {
        debug_assert!(self.cap > 0);
        // SAFETY: Requirements propegated into this function ones
        unsafe { deallocate(self.buffer_start, self.cap) }?;
        self.update_buffer(NonNull::dangling(), 0);
        Ok(())
    }

    /// Internal function that sets the capacity and raw buffer pointer.
    fn update_buffer(&mut self, ptr: NonNull<T>, cap: usize) {
        self.cap = cap;
        self.buffer_start = ptr;
    }
}

impl<T> Buffer for HeapBuffer<T> {
    type Element = T;

    fn capacity(&self) -> usize {
        self.cap
    }

    unsafe fn read_value(&mut self, index: usize) -> T {
        // SAFETY: it has the same requirements
        unsafe { self.read(index) }
    }

    unsafe fn write_value(&mut self, index: usize, value: T) {
        // SAFETY: [`Buffer::write_value`] ensures that the position is valid
        // and empty.
        let dst = unsafe { self.mut_ptr(index) };
        // SAFETY: [`PtrBuffer::mut_ptr`] ensures that the pointer is valid.
        // [`Buffer::write_value`] ensures that the position is empty.
        unsafe { ptr::write(dst, value) };
    }

    unsafe fn manually_drop(&mut self, index: usize) {
        // SAFETY: [`Buffer::manually_drop`] ensures that the position is valid
        // and filled.
        let to_drop = unsafe { self.mut_ptr(index) };
        // SAFETY: [`PtrBuffer::mut_ptr`] ensures that the pointer is valid.
        // [`Buffer::write_value`] ensures that the position is filled.
        unsafe { ptr::drop_in_place(to_drop) };
    }

    unsafe fn try_grow(&mut self, target: usize) -> Result<(), ResizeError> {
        if self.cap == 0 {
            // SAFETY: `self.cap` is checked in the conditional.
            // [`Buffer::try_grow`] ensures that `target` > `self.cap` (which is
            // 0)
            unsafe { self.allocate_array(target) }
        } else {
            // SAFETY: `self.cap` is checked to be grater than 0, which means
            // that `self.buffer_start` is not dangling.
            // [`Buffer::try_grow`] ensures that `target` > `self.cap` (which
            // implies `target` != `self.cap`)
            unsafe { self.resize_array(target) }
        }
    }

    unsafe fn try_shrink(&mut self, target: usize) -> Result<(), ResizeError> {
        if target == 0 {
            // SAFETY: [`Buffer::try_shrink`] ensures `target` < `self.cap`.
            // This means that `self.cap` > 0 (conditional) and thus
            // `self.buffer_start` is not dangling.
            unsafe { self.deallocate_array() }
        } else {
            // SAFETY: `target` is not 0 and it only allows positive values,
            // thus `target` > 0 at this point.
            // [`Buffer::try_shrink`] ensures `target` < `self.cap`. This means
            // that `target` != `self.cap`. Also `self.cap` > 0 (conditional)
            // and thus `self.buffer_start` is not dangling.
            unsafe { self.resize_array(target) }
        }
    }
}

impl<T: Copy> CopyValueBuffer for HeapBuffer<T> {
    unsafe fn copy_value(&self, index: usize) -> T {
        // SAFETY: it has the same requirements
        unsafe { self.read(index) }
    }
}

impl<T> PtrBuffer for HeapBuffer<T> {
    type ConstantPointer = *const T;
    type MutablePointer = *mut T;

    unsafe fn ptr(&self, index: usize) -> *const T {
        debug_assert!(index < self.capacity());
        let ptr = self.buffer_start.as_ptr();

        // SAFETY: `ptr` is at the start, `ptr.add(index)` points to the array's
        // position. [`PtrBuffer::ptr`] requires that the index is valid and
        // filled. Thus the pointer also is.
        unsafe { ptr.add(index) }
    }

    unsafe fn mut_ptr(&mut self, index: usize) -> *mut T {
        debug_assert!(index < self.capacity());
        let ptr = self.buffer_start.as_ptr();

        // SAFETY: `ptr` is at the start, `ptr.add(index)` points to the array's
        // position. [`PtrBuffer::mut_ptr`] requires that the index is valid and
        // filled. Thus the pointer also is.
        unsafe { ptr.add(index) }
    }
}

impl<T> RefBuffer for HeapBuffer<T> {
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

impl<T> ContiguousMemoryBuffer for HeapBuffer<T> {}

impl<T> Default for HeapBuffer<T> {
    fn default() -> Self {
        Self::new()
    }
}

// SAFETY: As a buffer it's not its responsabilities to clean the values that it
// saves. The container should use [`Buffer::manually_drop`] and
// [`Buffer::manually_drop_range`] to properly drop the values it contains.
unsafe impl<#[may_dangle] T> Drop for HeapBuffer<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            // SAFETY: At this point all content should have been dropped
            unsafe {
                // Even if it fails, we can only ignore the error
                let _ = self.deallocate_array();
            }
        }
    }
}

/// Tries to allocate a new array of a given size on the heap.
///
/// # Safety
///   * `size` must be bigger than zero.
unsafe fn try_array_alloc<T>(size: usize) -> Result<NonNull<T>, ResizeError> {
    debug_assert!(size > 0);
    let layout = Layout::array::<T>(size)?;
    // SAFETY: Because `try_array_alloc` ensures that `size` > 0, `layout` is
    // valid to allocate.
    let ptr = unsafe { std::alloc::alloc(layout) };
    let ptr = ptr as *mut T;
    NonNull::new(ptr).ok_or(ResizeError::OutOfMemory)
}

/// Tries to reallocate an existing array (growing or shrinking).
///
/// # SAFETY
///   * `new_size` must be bigger than zero.
///   * `new_size` must be different than `old_size`.
unsafe fn try_array_realloc<T>(
    old_ptr: NonNull<T>,
    old_size: usize,
    new_size: usize,
) -> Result<NonNull<T>, ResizeError> {
    debug_assert!(new_size > 0);
    debug_assert!(old_size != new_size);

    let old_layout = Layout::array::<T>(old_size)?;
    let new_layout = Layout::array::<T>(new_size)?;

    let old_ptr = old_ptr.as_ptr() as *mut u8;

    // SAFETY:
    //  * It only uses this allocator (global).
    //  * Layout is recreated by reading `self.cap` (alignment depends on type,
    //    which is constant).
    //  * `new_size` > 0 because of this function preconditions.
    //  * The new size is managed by [`Layout`], which ensures its safety.
    let new_ptr = unsafe { std::alloc::realloc(old_ptr, old_layout, new_layout.size()) };
    let new_ptr = new_ptr as *mut T;

    NonNull::new(new_ptr).ok_or(ResizeError::OutOfMemory)
}

/// Tries to deallocate an existing array.
///
/// # SAFETY
///   * `size` must be bigger than zero.
///   * `size` must be the current size of the array to deallocate.
///   * `ptr` must point the head of the array to deallocate.
unsafe fn deallocate<T>(ptr: NonNull<T>, size: usize) -> Result<(), ResizeError> {
    debug_assert!(size > 0);
    let layout = Layout::array::<T>(size)?;
    let ptr = ptr.as_ptr();
    let ptr = ptr as *mut u8;

    // SAFETY:
    //  * It only uses this allocator (global).
    //  * The number of elements (size) must be the current as per the
    //    precondition.
    //  * The new size is managed by [`Layout`], which ensures its safety.
    unsafe { std::alloc::dealloc(ptr, layout) };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_grow_from_default() {
        const TARGET: usize = 1;

        let mut buffer = HeapBuffer::<i32>::new();

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

        let mut buffer = HeapBuffer::<i32>::new();

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

        let mut buffer = HeapBuffer::<i32>::new();

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

        let mut buffer = HeapBuffer::<i32>::new();

        // SAFETY: 0 == TARGET2 < TARGET1
        unsafe {
            buffer.try_grow(TARGET1).unwrap();
            buffer.try_shrink(TARGET2).unwrap();
        }

        assert!(buffer.capacity() < TARGET1);
        assert!(buffer.capacity() == TARGET2);
    }
}
