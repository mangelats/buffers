#![feature(dropck_eyepatch)]
#![feature(maybe_uninit_uninit_array)]

use base_buffers::heap::HeapBuffer;
use composites::{svo::SvoBuffer, zsto::ZstOptBuffer};

#[path = "1_interface/_mod.rs"]
pub mod interface;

#[path = "2_base_buffers/_mod.rs"]
pub mod base_buffers;

#[path = "3_composites/_mod.rs"]
pub mod composites;

pub type DefaultBuffer<T> = ZstOptBuffer<T, SvoBuffer<T, HeapBuffer<T>, 256>>;
