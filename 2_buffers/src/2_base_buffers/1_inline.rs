pub struct InlineBuffer<T, const SIZE: usize> {
    array: [T; SIZE],
}
