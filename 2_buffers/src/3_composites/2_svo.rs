use crate::{base_buffers::inline::InlineBuffer, interface::Buffer};

pub struct SvoBuffer<T, B: Buffer<T>, const SMALL_SIZE: usize> {
    small: InlineBuffer<T, SMALL_SIZE>,
    big: B,
}
