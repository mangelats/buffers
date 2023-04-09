#![feature(dropck_eyepatch)]
#![feature(maybe_uninit_uninit_array)]

#[path = "1_interface/_mod.rs"]
pub mod interface;

#[path = "2_base_buffers/_mod.rs"]
pub mod base_buffers;

#[path = "a_test_utils/_mod.rs"]
pub(crate) mod test_utils;
