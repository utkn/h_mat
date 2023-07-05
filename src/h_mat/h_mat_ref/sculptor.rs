use std::marker::PhantomData;

use crate::h_mat::{AccessRow, HMat};

use super::HMatRef;

pub struct SculptDirective<Head, Tail, Access>(
    PhantomData<Head>,
    PhantomData<Tail>,
    PhantomData<Access>,
);

pub trait Sculptor<'a, Head, Tail, Access, T, R> {
    type Rem;
    fn sculpt(h: &'a HMat<T, R>) -> HMatRef<Head, Self::Rem>;
}

impl<'a, D, A, T, R> Sculptor<'a, D, (), A, T, R> for HMatRef<'a, D, ()>
where
    HMat<T, R>: AccessRow<D, A>,
{
    type Rem = ();

    fn sculpt(h: &'a HMat<T, R>) -> HMatRef<D, Self::Rem> {
        HMatRef {
            row: h.get_row_ref(),
            rem: (),
        }
    }
}

impl<'a, T1, T2, R1, Tr, A1, A2, T, R> Sculptor<'a, T1, SculptDirective<T2, Tr, A2>, A1, T, R>
    for HMatRef<'a, T1, HMatRef<'a, T2, R1>>
where
    HMatRef<'a, T2, R1>: Sculptor<'a, T2, Tr, A2, T, R>,
    HMat<T, R>: AccessRow<T1, A1>,
    HMat<T, R>: AccessRow<T2, A2>,
{
    type Rem = HMatRef<'a, T2, <HMatRef<'a, T2, R1> as Sculptor<'a, T2, Tr, A2, T, R>>::Rem>;

    fn sculpt(h: &'a HMat<T, R>) -> HMatRef<T1, Self::Rem> {
        HMatRef {
            row: h.get_row_ref(),
            rem: HMatRef::<'a, T2, R1>::sculpt(h),
        }
    }
}
