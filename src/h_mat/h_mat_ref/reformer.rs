use std::marker::PhantomData;

use crate::h_mat::{AccessRow, HMat};

use super::HMatRef;

pub struct ReformableDirective<Head, Tail, Access>(
    PhantomData<*const Head>,
    PhantomData<*const Tail>,
    PhantomData<*const Access>,
);

pub trait Reformable<'a, Head, Directive, Access> {
    type Rem;
    fn reform(&'a self) -> HMatRef<'a, Head, Self::Rem>;
}

impl<'a, D, A, T, R> Reformable<'a, D, (), A> for HMat<T, R>
where
    D: 'static,
    HMat<T, R>: AccessRow<D, A>,
{
    type Rem = ();

    fn reform(&'a self) -> HMatRef<'a, D, Self::Rem> {
        HMatRef {
            row: self.get_row_ref(),
            rem: (),
        }
    }
}

impl<'a, D1, D2, A1, A2, R2, T, R> Reformable<'a, D1, ReformableDirective<D2, R2, A2>, A1>
    for HMat<T, R>
where
    D1: 'static,
    D2: 'static,
    HMat<T, R>: AccessRow<D1, A1>,
    HMat<T, R>: Reformable<'a, D2, R2, A2>,
    R: AccessRow<D2, A2>,
{
    type Rem = HMatRef<'a, D2, <Self as Reformable<'a, D2, R2, A2>>::Rem>;

    fn reform(&'a self) -> HMatRef<'a, D1, Self::Rem> {
        HMatRef {
            row: self.get_row_ref(),
            rem: self.reform(),
        }
    }
}
