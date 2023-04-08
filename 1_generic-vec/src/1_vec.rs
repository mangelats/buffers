use std::marker::PhantomData;

pub struct Vector<T> {
    _m: PhantomData<T>,
}

impl<T> Vector<T> {
    pub fn new() -> Vector<T> {
        Vector { _m: PhantomData }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vector = Vector::<u32>::new();
    }
}
