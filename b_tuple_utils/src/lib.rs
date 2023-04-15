#![feature(tuple_trait)]
use std::marker::Tuple;

pub trait TupleExt: Tuple {
    type Ref<'a>;

    fn as_ref<'a>(&'a self) -> Ref<'a>;
}
