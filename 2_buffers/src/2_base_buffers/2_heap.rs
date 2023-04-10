use std::marker::PhantomData;

pub struct HeapBuffer<T> {
    _m: PhantomData<T>,
}
