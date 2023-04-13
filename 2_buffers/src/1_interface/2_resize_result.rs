use std::alloc::LayoutError;

/// Errors that may happen when attempting to resize a buffer
#[derive(Debug, Clone)]
pub enum ResizeError {
    /// This buffer will never support this operation
    /// (eg. trying to resize an inline buffer)
    UnsupportedOperation,

    /// This buffer doesn't support this operation for this type
    /// (eg. cannot define a memory layout)
    UnsupportedType,

    OutOfMemory,
}

impl From<LayoutError> for ResizeError {
    fn from(_: LayoutError) -> Self {
        Self::UnsupportedType
    }
}
