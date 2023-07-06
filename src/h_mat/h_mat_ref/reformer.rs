use std::marker::PhantomData;

use crate::h_mat::AccessRow;

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

// Receiver T for lower priority when called with (&&T).
impl<'a, D, A, T> Reformable<'a, D, (), A> for T
where
    D: 'static,
    T: AccessRow<D, A>,
{
    type Rem = ();

    fn reform(&'a self) -> HMatRef<'a, D, Self::Rem> {
        HMatRef {
            row: self.get_row_ref(),
            rem: (),
        }
    }
}

// Receiver &T for higher priority when called with (&&T).
impl<'a, D1, D2, A1, A2, R2, T> Reformable<'a, D1, ReformableDirective<D2, R2, A2>, A1> for &T
where
    D1: 'static,
    D2: 'static,
    T: AccessRow<D1, A1>,
    T: Reformable<'a, D2, R2, A2>,
{
    type Rem = HMatRef<'a, D2, <T as Reformable<'a, D2, R2, A2>>::Rem>;

    fn reform(&'a self) -> HMatRef<'a, D1, Self::Rem> {
        HMatRef {
            row: self.get_row_ref(),
            rem: (*self).reform(),
        }
    }
}
