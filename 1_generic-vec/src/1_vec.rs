use std::marker::PhantomData;

pub struct Vector<T> {
    _m: PhantomData<T>,
}

impl<T> Vector<T> {
    pub fn new() -> Vector<T> {
        Vector { _m: PhantomData }
    }

    pub fn len(self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vector_should_be_build_with_new() {
        let _vector = Vector::<u32>::new();
    }

    #[test]
    fn empty_vector_should_have_no_length() {}
}
