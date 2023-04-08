/// Low level of abstraction of multiple instances of data of type `T` managed as a group.
///
/// This is perticularly useful to allow different ways of managing data in memory with a uniform interface.
///
/// ## Safety
/// Buffers are not responsible for a lot of safety features that one may expect (like dropping the values on
/// drop, check boundaries, check if the memory is initialized, and so on). This is because implementations may
/// ensure safety by design instead of adding checks every time. A lot of times the buffer doesn't have the
/// information anyways, making the check hard or impossible. In practice this makes this trait and most of its
/// methods unsafe.
///
/// ## Notes
/// This interface has been deliberately designed to have a little constrains to the implementations as possible.
/// For example: the underlying data doesn't need to be saved in a contiguous chunk of memory, and it could be on
/// the stack, on the heap, etc.
pub trait Buffer<T> {}
