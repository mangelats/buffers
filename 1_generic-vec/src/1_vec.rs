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
    fn it_works() {
        let _vector = Vector::<u32>::new();
    }
}
