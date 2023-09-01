/// Errors that may happen when attempting to resize a buffer.
#[derive(Debug, Clone)]
pub enum ResizeError {
    /// The underlying mechanism to aquire memory cannot aquire more.
    ///
    /// For example: you filled all the memory in your system.
    OutOfMemory,

    /// The buffer cannot grow that much because because it would surpass the
    /// theoretical limits of the system.
    ///
    /// For example: you are trying to grow to more bytes than the supported by
    /// the architecture.
    TheoreticalLimitSurpassed,

    /// This buffer cannot perform the specified resizing operation due to its
    /// properties.
    ///
    /// An example is [`crate::base_buffers::inline::InlineBuffer`]: it's
    /// fixed-sized, so no matter what both `try_grow` and `try_shrink` will
    /// fail.
    UnsupportedOperation,

    /// This buffer cannot perform the specified resizing operation due to some
    /// error, but due to the buffer's setup it cannot provide more information.
    ///
    /// An example is [`crate::base_buffers::allocator::AllocatorBuffer`]: the
    /// allocator API only has a generic error for failing, giving no further
    /// details on why that happened.
    UndistinguishableError,
}

/// Automatic transformation from [`std::alloc::LayoutError`] to
/// [`ResizeError`].
///
/// A layout error means that it tries to allocate something impossible
/// thoretically.
impl From<std::alloc::LayoutError> for ResizeError {
    fn from(_: std::alloc::LayoutError) -> Self {
        Self::TheoreticalLimitSurpassed
    }
}

/// Automatic transformation from [`std::alloc::AllocError`] to
/// [`ResizeError`].
///
/// By definition an [`std::alloc::AllocError`] has an unknown underlying
/// reason.
#[cfg(feature = "allocator")]
impl From<std::alloc::AllocError> for ResizeError {
    fn from(_: std::alloc::AllocError) -> Self {
        Self::UndistinguishableError
    }
}
