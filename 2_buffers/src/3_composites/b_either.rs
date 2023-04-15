use std::marker::PhantomData;

use crate::interface::Buffer;

pub enum EitherBuffer<T, A: Buffer<T>, B: Buffer<T>> {
    /// First option (A buffer)
    First(A),
    /// Second option (B buffer)
    Second(B),

    /// Internal option that never can be selected which holds PhantomData to T
    _InternalMarker(Never, PhantomData<T>),
}

/// A type that can never exist.
///
/// Equivalent to the never type (!) which is experimental
/// (see issue #35121 <https://github.com/rust-lang/rust/issues/35121>).
pub enum Never {}
