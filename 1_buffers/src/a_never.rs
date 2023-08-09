/// A type that can never exist.
///
/// This also applies to any structure or an enum value with it. It's useful to
/// tell the compiler that a struct or enum value will never be used.
///
/// Equivalent to the never type (!) which is experimental
/// (see issue #35121 <https://github.com/rust-lang/rust/issues/35121>).
pub enum Never {}
