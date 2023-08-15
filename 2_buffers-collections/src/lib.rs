#[path = "1_vec.rs"]
pub mod vec;
pub use vec::Vector;

// Force running README.md example code, so we can ensure it actually works :)
#[doc = include_str!("../../README.md")]
#[cfg(doctest)]
extern "C" {}
