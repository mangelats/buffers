/// Errors that may happen when attempting to resize a buffer
#[derive(Debug, Clone)]
pub enum ResizeError {
    UnsupportedOperation,
}

pub type ResizeResult<T = ()> = Result<T, ResizeError>;
