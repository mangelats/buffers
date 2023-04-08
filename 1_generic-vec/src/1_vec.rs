use std::marker::PhantomData;

pub struct Vec<T> {
    _m: PhantomData<T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
