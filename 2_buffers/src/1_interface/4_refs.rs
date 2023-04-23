use super::{ptrs::PtrBuffer, Buffer};

/// Represents a buffer which has pointers to its elements
///
/// Note that in some cases the elements themselves may not ahve a unique pointers (eg. zero-sized types)
pub trait RefBuffer: Buffer {
    type ConstantReference<'a>
    where
        Self: 'a;
    type MutableReference<'a>
    where
        Self: 'a;

    /// Get a contant pointer to the value in the specified index.
    ///
    /// # SAFETY
    /// `index` needs to be in bounds (`0 <= index < capacity`). It's undefined behaviour when not.
    ///
    /// The pointer may point to unitialized or garbage data. It's the responsability of the caller to keep track of the state.
    unsafe fn index(&self, index: usize) -> Self::ConstantReference<'_>;

    /// Get a mutable pointer to the value in the specified index.
    ///
    /// # SAFETY
    /// `index` needs to be in bounds (`0 <= index < capacity`). It's undefined behaviour when not.
    ///
    /// The pointer may point to unitialized or garbage data. It's the responsability of the caller to keep track of the state.
    unsafe fn mut_index(&mut self, index: usize) -> Self::MutableReference<'_>;
}

pub trait DefaultRefBuffer:
    PtrBuffer<
    ConstantPointer = *const <Self as Buffer>::Element,
    MutablePointer = *mut <Self as Buffer>::Element,
>
{
}

impl<B> RefBuffer for B
where
    B: DefaultRefBuffer,
{
    type ConstantReference<'a> = &'a Self::Element
    where
        Self: 'a;
    type MutableReference<'a> = &'a mut Self::Element
    where
        Self: 'a;

    unsafe fn index(&self, index: usize) -> Self::ConstantReference<'_> {
        &*self.ptr(index)
    }

    unsafe fn mut_index(&mut self, index: usize) -> Self::MutableReference<'_> {
        &mut *self.mut_ptr(index)
    }
}
