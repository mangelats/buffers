use super::{ptrs::PtrBuffer, Buffer};

/// This trait extends the buffers that have the hability to generate references
/// to an element in the buffer. This reference may be a regular rust reference
/// (`&T`) but it doesn't have to. For example: in a structure of arrays setting
/// it could be a structure of references.
///
/// If the buffer already implements [`PtrBuffer`] it may be able to be
/// automatically implemented using [`DefaultRefBuffer`].
pub trait RefBuffer: Buffer {
    /// Type representing a reference to [`Buffer::Element`] with `'a` lifetime.
    type ConstantReference<'a>
    where
        Self: 'a;

    /// Type representing a mutable reference to [`Buffer::Element`] with `'a`
    /// lifetime.
    type MutableReference<'a>
    where
        Self: 'a;

    /// Get a reference to the element in the specified position.
    ///
    /// # Safety
    ///   * `index` must be a valid position.
    ///   * Position `index` must be filled.
    unsafe fn index(&self, index: usize) -> Self::ConstantReference<'_>;

    /// Get a mutable reference to the element in the specified position.
    ///
    /// # Safety
    ///   * `index` must be a valid position.
    ///   * Position `index` must be filled.
    unsafe fn mut_index(&mut self, index: usize) -> Self::MutableReference<'_>;
}

/// Helper trait which has a blanket implementation of [`RefBuffer`] for buffers
/// which are [`PtrBuffer`] and its pointers are the regular rust ones
/// (`*const T` and `*mut T`).
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
