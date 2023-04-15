use std::marker::PhantomData;

use crate::{base_buffers::zst::ZstBuffer, interface::Buffer};

use super::conditional::{ConditionalBuffer, Selector};

/// Composite buffer that automatically uses a ZstBuffer when T is a ZST.
pub type ZstOptBuffer<T, B> = ConditionalBuffer<T, ZstBuffer<T>, B, ZstSelector<T>>;

pub struct ZstSelector<T>(PhantomData<T>);
impl<T> Selector for ZstSelector<T> {
    const SELECT_A: bool = std::mem::size_of::<T>() == 0;
}
