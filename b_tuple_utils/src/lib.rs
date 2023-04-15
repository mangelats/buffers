#![feature(tuple_trait)]
use std::marker::Tuple;

pub trait TupleExt: Tuple {
    type Ref<'a>;
    fn as_ref<'a>(&'a self) -> Ref<'a>;

    type MutRef<'a>;
    fn as_mut_ref(&mut self) -> MutRef<'_>;
}

impl<T0, T1, T2> TupleExt for (T1, T2, T3) {
    type Ref<'a> = (&'a T1, &'a T2, &'a T3);
    fn as_ref(&self) -> Ref<'_> {
        (&self.0, &self.1, &self.2)
    }

    type MutRef<'a> = (&'a mut T1, &'a mut T2, &'a mut T3);
    fn as_mut_ref(&mut self) -> MutRef<'_> {
        (&mut self.0, &mut self.1, &mut self.2)
    }
}
