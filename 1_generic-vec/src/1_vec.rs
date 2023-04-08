use std::marker::PhantomData;

pub struct Vec<T> {
    _m: PhantomData<T>,
}
