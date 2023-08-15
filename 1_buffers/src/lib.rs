#![feature(dropck_eyepatch)]
#![feature(maybe_uninit_uninit_array)]
#![cfg_attr(feature = "allocator", feature(allocator_api))]

use base_buffers::heap::HeapBuffer;
use composites::{svo::SvoBuffer, zsto::ZstoBuffer};

#[path = "1_interface/_mod.rs"]
pub mod interface;

#[path = "2_base_buffers/_mod.rs"]
pub mod base_buffers;

#[path = "3_composites/_mod.rs"]
pub mod composites;

#[path = "a_never.rs"]
pub mod never;

#[path = "b_narrow_ref.rs"]
pub mod narrow_ref;

pub type DefaultBuffer<T, const SMALL_VECTOR_SIZE: usize = 256> =
    ZstoBuffer<SvoBuffer<SMALL_VECTOR_SIZE, HeapBuffer<T>>>;
