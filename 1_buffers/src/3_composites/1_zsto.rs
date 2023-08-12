use crate::{base_buffers::zst::ZstBuffer, never::PhantomNever};

use super::conditional::{ConditionalBuffer, Selector};

/// Composite buffer that automatically uses a ZstBuffer when T is a ZST.
pub type ZstOptBuffer<T, B> = ConditionalBuffer<T, ZstBuffer<T>, B, ZstSelector<T>>;

/// Selector that detects if T is a ZST
pub struct ZstSelector<T>(PhantomNever<T>);
impl<T> Selector for ZstSelector<T> {
    const SELECT_A: bool = std::mem::size_of::<T>() == 0;
}
