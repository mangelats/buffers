use std::alloc::Allocator;

pub struct AllocatorBuffer<T, A: Allocator> {
    alloc: A,
    ptr: NonNull<T>,
    cap: usize,
    _marker: PhantomData<T>,
}
