//! This module contains the base buffers, or the buffers that contain data by
//! themselves.

#[path = "1_inline.rs"]
pub mod inline;
pub use inline::InlineBuffer;

#[path = "2_heap.rs"]
pub mod heap;
pub use heap::HeapBuffer;

#[path = "3_zst.rs"]
pub mod zst;
pub use zst::ZstBuffer;

#[path = "4_slice.rs"]
pub mod slice;
pub use slice::SliceBuffer;

#[cfg(feature = "allocator")]
#[path = "5_allocator.rs"]
pub mod allocator;
#[cfg(feature = "allocator")]
pub use allocator::AllocatorBuffer;
