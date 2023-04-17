#![feature(tuple_trait)]
use std::marker::Tuple;

pub trait TupleExt: Tuple {
    type MapType<M: TypeMap>;

    // fn map<M>(&self, mapper: M) -> Self::MapType<M>
    // where
    //     M: TuppleMapper;
}

pub trait TypeMap {
    type Output<T>;
}

pub trait TuppleMapper: TypeMap {
    fn map_single<T>(value: &T) -> Self::Output<T>;
}

impl TupleExt for () {
    type MapType<M: TypeMap> = ();

    // fn map<M>(&self, _mapper: M) -> Self::MapType<M>
    // where
    //     M: TuppleMapper,
    // {
    //     ()
    // }
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
