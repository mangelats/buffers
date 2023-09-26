/// Utility trait that is used to explicitly narrow the lifetime of a reference.
pub trait NarrowRef<'a, T: ?Sized> {
    fn narrow_ref(self) -> &'a T;
}

impl<'original: 'part, 'part, T: ?Sized> NarrowRef<'part, T> for &'original T {
    fn narrow_ref(self) -> &'part T {
        self
    }
}

/// Utility trait that is used to explicitly narrow the lifetime of a reference.
pub trait NarrowMutRef<'a, T: ?Sized> {
    fn narrow_mut_ref(self) -> &'a mut T;
}

impl<'original: 'part, 'part, T: ?Sized> NarrowMutRef<'part, T> for &'original mut T {
    fn narrow_mut_ref(self) -> &'part mut T {
        self
    }
}
