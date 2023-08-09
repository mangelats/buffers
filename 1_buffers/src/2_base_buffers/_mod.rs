//! This module contains the base buffers, or the buffers that contain data by
//! themselves.

#[path = "1_inline.rs"]
pub mod inline;

#[path = "2_heap.rs"]
pub mod heap;

#[path = "3_zst.rs"]
pub mod zst;

#[cfg(feature = "allocator")]
#[path = "4_allocator.rs"]
pub mod allocator;
