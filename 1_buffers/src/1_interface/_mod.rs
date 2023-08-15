//! This module contains all the abstractions which define how a buffer and
//! other closely-related values and abstractions.

#[path = "1_buffer.rs"]
pub mod buffer;
pub use self::buffer::Buffer;

#[path = "2_resize_error.rs"]
pub mod resize_error;
pub use self::resize_error::ResizeError;

#[path = "3_ptrs.rs"]
pub mod ptrs;

#[path = "4_refs.rs"]
pub mod refs;

#[path = "5_contiguous_memory.rs"]
pub mod contiguous_memory;

#[path = "6_indirect_buffer.rs"]
pub mod indirect_buffer;
