use std::alloc::Allocator;

pub struct AllocatorBuffer<T, A: Allocator> {
    ptr: NonNull<T>,
    cap: usize,
    alloc: A,
    _marker: PhantomData<T>,
}
