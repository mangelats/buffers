//! This module contains buffers which use one or more base buffers but change
//! their behaviour. This may be optimizations, change how the buffer grows,
//! etc.

#[path = "1_zsto.rs"]
pub mod zsto;
pub use zsto::ZstoBuffer;

#[path = "2_svo.rs"]
pub mod svo;
pub use svo::SvoBuffer;

#[path = "3_exponential_growth.rs"]
pub mod exponential_growth;
pub use exponential_growth::ExponentialGrowthBuffer;

#[path = "4_at_least.rs"]
pub mod at_least;
pub use at_least::AtLeastBuffer;

#[cfg(feature = "array")]
#[path = "5_array.rs"]
pub mod array;
#[cfg(feature = "array")]
pub use array::ArrayBuffer;

#[path = "a_conditional.rs"]
pub mod conditional;

#[path = "b_either.rs"]
pub mod either;

#[path = "c_grow_mock.rs"]
pub mod grow_mock;
