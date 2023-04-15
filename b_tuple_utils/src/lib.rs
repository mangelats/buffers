#![feature(tuple_trait)]
use std::marker::Tuple;

pub trait TypeMap {
    type Output<T>;
}

// pub trait<U> Includes<U>: Tuple {}

// pub trait Reducer<R, T: Tuple> {
//     fn reduce<U>(current: R, value: U) -> R where T: Includes<U>;
// }

pub trait TupleExt: Tuple {
    type Map<M: TypeMap>;
}

// impl<T0, T1, T2> TupleExt for (T0, T1, T2) {
//     type Map<M: TypeMap> = (M::Output<T0>, M::Output<T1>, M::Output<T2>);
// }

macro_rules! impl_tuple_ext {
    () => {};
    ($t:ident) => {
        impl<$t> TupleExt for ($t,) {
            type Map<M: TypeMap> = (M::Output<$t>,);
        }
    };
}

impl_tuple_ext! {T0}

// impl<T0, T1, T2> Includes<T0> for (T0, T1, T2) {}
// impl<T0, T1, T2> Includes<T1> for (T0, T1, T2) {}
// impl<T0, T1, T2> Includes<T2> for (T0, T1, T2) {}

// pub trait Reducer<R, T: Tuple> {
//     fn reduce<U>(current: R, value: U) -> R where T: Includes<U>;
// }
