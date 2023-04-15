use std::alloc::Allocator;

/// Similar buffer to HeapBuffer but it uses Allocators instead
pub struct AllocatorBuffer<T, A: Allocator> {
    ptr: NonNull<T>,
    cap: usize,
    alloc: A,
    _marker: PhantomData<T>,
}
