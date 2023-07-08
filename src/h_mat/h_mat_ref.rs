use crate::{AccessColRef, AccessRowDirective, AccessRowRef, HCol, ReformDirective, Reformer};

use super::Row;

/// A reference to a `HMat` with arbitrarily ordered rows. Invoke `reform` on the original `HMat` to construct a `HMatRef`.
#[derive(Clone, Copy, Debug)]
pub struct HMatRef<'a, D, R> {
    pub row: &'a Row<D>,
    pub rem: R,
}

impl<'a, D, R> AccessRowRef<D, ()> for HMatRef<'a, D, R> {
    fn get_row_ref(&self) -> &Row<D> {
        &self.row
    }
}

impl<'a, T, R, D, A> AccessRowRef<D, AccessRowDirective<A>> for HMatRef<'a, T, R>
where
    R: AccessRowRef<D, A>,
{
    fn get_row_ref(&self) -> &Row<D> {
        self.rem.get_row_ref()
    }
}

impl<'a, T1, T2, R> AccessColRef<'a, T1> for HMatRef<'a, T1, HMatRef<'a, T2, R>>
where
    HMatRef<'a, T2, R>: AccessColRef<'a, T2>,
{
    type Rem = HCol<&'a T2, <HMatRef<'a, T2, R> as AccessColRef<'a, T2>>::Rem>;
    fn get_col_ref(&'a self, idx: usize) -> HCol<&T1, Self::Rem> {
        HCol {
            elem: self.row.get(idx),
            rem: self.rem.get_col_ref(idx),
        }
    }
}

impl<'a, T> AccessColRef<'a, T> for HMatRef<'a, T, ()> {
    type Rem = ();
    fn get_col_ref(&'a self, idx: usize) -> HCol<&T, Self::Rem> {
        HCol {
            elem: self.row.get(idx),
            rem: (),
        }
    }
}

impl<'a, H, D, A> Reformer<'a, H, D, (), A> for HMatRef<'a, D, ()>
where
    H: AccessRowRef<D, A>,
{
    fn reform(h: &'a H) -> Self {
        HMatRef {
            row: h.get_row_ref(),
            rem: (),
        }
    }
}

impl<'a, H, D1, D2, R, Dr, A1, A2> Reformer<'a, H, D1, ReformDirective<D2, Dr, A2>, A1>
    for HMatRef<'a, D1, HMatRef<'a, D2, R>>
where
    H: AccessRowRef<D1, A1>,
    HMatRef<'a, D2, R>: Reformer<'a, H, D2, Dr, A2>,
{
    fn reform(h: &'a H) -> Self {
        HMatRef {
            row: h.get_row_ref(),
            rem: <HMatRef<'a, D2, R> as Reformer<'a, H, D2, Dr, A2>>::reform(h),
        }
    }
}
