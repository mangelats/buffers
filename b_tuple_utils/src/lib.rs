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
    type Ref<'a>
    where
        Self: 'a;
    fn as_ref<'a>(&'a self) -> Self::Ref<'a>;

    //     type MutRef<'a>;
    //     fn as_mut_ref(&mut self) -> MutRef<'_>;

    //     type ConstPtr;
    //     fn as_ptr(*const self) -> ConstPtr;

    //     type MutPtr;
    //     fn as_mut_ptr(*mut self) -> MutPtr;

    //     type Map<M: TypeMap>;

    //     fn reduce<R>(self, initial: R, r: Reducer<R, Self>) -> R;
}

// impl<T0, T1, T2> TupleExt for (T1, T2, T3) {
//     type Ref<'a> = (&'a T1, &'a T2, &'a T3);
//     fn as_ref(&self) -> Ref<'_> {
//         (&self.0, &self.1, &self.2)
//     }

//     type MutRef<'a> = (&'a mut T1, &'a mut T2, &'a mut T3);
//     fn as_mut_ref(&mut self) -> MutRef<'_> {
//         (&mut self.0, &mut self.1, &mut self.2)
//     }

//     type ConstPtr = (*const T0, *const T1, *const T2);
//     fn as_ptr(*const self) -> ConstPtr {
//         (&*self.0, &*self.1, &*self.2)
//     }

//     type MutPtr = (*mut T0, *mut T1, *mut T2);
//     fn as_mut_ptr(*mut self) -> MutPtr {
//         (&mut *self.0, &mut *self.1, &mut *self.2)
//     }

//     type Map<M: TypeMap> = (M::Output<T0>, M::Output<T1>, M::Output<T2>);

//     fn reduce<R>(self, initial: R, r: Reducer<R, Self>) -> R {
//         initial
//     }
// }

// impl<T0, T1, T2> Includes<T0> for (T0, T1, T2) {}
// impl<T0, T1, T2> Includes<T1> for (T0, T1, T2) {}
// impl<T0, T1, T2> Includes<T2> for (T0, T1, T2) {}

// pub trait Reducer<R, T: Tuple> {
//     fn reduce<U>(current: R, value: U) -> R where T: Includes<U>;
// }
