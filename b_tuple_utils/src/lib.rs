#![feature(tuple_trait)]
use std::marker::Tuple;

pub trait TupleExt: Tuple {
    type Ref<'a>;

    fn as_ref<'a>(&'a self) -> Ref<'a>;
}

impl<T0, T1, T2> TupleExt for (T1, T2, T3) {
    type Ref<'a> = (&'a T1, &'a T2, &'a T3);

    fn as_ref(&self) -> Ref<'_> {
        (&self.0, &self.1, &self.2)
    }
}
