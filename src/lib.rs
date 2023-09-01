#![feature(dropck_eyepatch)]
#![feature(maybe_uninit_uninit_array)]
#![cfg_attr(feature = "allocator", feature(allocator_api))]
#![cfg_attr(feature = "array", feature(maybe_uninit_array_assume_init))]

use base_buffers::heap::HeapBuffer;
use composites::{svo::SvoBuffer, zsto::ZstoBuffer};

#[path = "1_interface/_mod.rs"]
pub mod interface;

#[path = "2_base_buffers/_mod.rs"]
pub mod base_buffers;

#[path = "3_composites/_mod.rs"]
pub mod composites;

#[path = "4_collections/_mod.rs"]
pub mod collections;

#[path = "a_test_utils/_mod.rs"]
pub mod test_utils;

#[path = "b_never.rs"]
pub mod never;

#[path = "c_narrow_ref.rs"]
pub mod narrow_ref;

/// Default buffer composition.
///
/// It's meant to be used as a sensible default for most cases. Its composition
/// may change, specially when improving performance. If it doesn't comfort your
/// use case, make one which is! (that's what this library is about)
pub type DefaultBuffer<T> = ZstoBuffer<SvoBuffer<256, HeapBuffer<T>>>;

// Force running README.md example code, so we can ensure it actually works :)
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
extern "C" {}
