use std::marker::PhantomData;

use crate::interface::Buffer;

pub struct ZstBuffer<T> {
    _m: PhantomData<T>,
}
