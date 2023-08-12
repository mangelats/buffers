use crate::{base_buffers::zst::ZstBuffer, interface::Buffer, never::PhantomNever};

use super::conditional::{ConditionalBuffer, Selector};

/// Composite buffer that automatically uses a ZstBuffer when T is a ZST.
pub type ZstoBuffer<B> =
    ConditionalBuffer<ZstBuffer<<B as Buffer>::Element>, B, ZstSelector<<B as Buffer>::Element>>;

/// Selector that detects if T is a ZST
pub struct ZstSelector<T>(PhantomNever<T>);
impl<T> Selector for ZstSelector<T> {
    const SELECT_A: bool = std::mem::size_of::<T>() == 0;
}
