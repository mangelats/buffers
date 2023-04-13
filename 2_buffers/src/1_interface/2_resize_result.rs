/// Errors that may happen when attempting to resize a buffer
#[derive(Debug, Clone)]
pub enum ResizeError {
    UnsupportedOperation,
    UnsupportedType,
}
