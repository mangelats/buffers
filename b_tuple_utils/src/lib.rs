#![feature(tuple_trait)]
use std::marker::Tuple;

pub trait TypeMap {
    type Output<T>;
}

pub trait TupleExt: Tuple {
    type MapType<M: TypeMap>;
}
impl TupleExt for () {
    type MapType<M: TypeMap> = ();
}

macro_rules! impl_tuple_map {
    (() -> ($($body:path,)*)) => { ($($body),*, ) };
    (($t:ident,$($rest:ident,)*) -> ($($body:path,)*)) => {
        impl_tuple_map!{ ($($rest,)*) -> (M::Output<$t>, $($body,)*) }
    };
}
macro_rules! impl_tuple_ext_one {
    ($($t:ident),+) => {
        impl<$($t),+> TupleExt for ($($t),+,) {
            type MapType<M: TypeMap> = impl_tuple_map!(($($t,)+) -> ());
        }
    };
}
macro_rules! impl_tuple_ext {
    () => {};
    ($t:ident, $($rest:ident),+) => {
        impl_tuple_ext_one! {$t, $($rest),+}
        impl_tuple_ext_one! {$($rest),+}
    };
}

impl_tuple_ext! {T0, T1, T2, T3, T4}
