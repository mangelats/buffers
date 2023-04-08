use std::mem::MaybeUninit;

pub struct InlineBuffer<T, const SIZE: usize> {
    array: [MaybeUninit<T>; SIZE],
}
