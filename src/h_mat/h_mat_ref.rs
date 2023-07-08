use std::marker::PhantomData;

use crate::{
    AccessColRef, AccessRowDirective, AccessRowRef, HCol, HMatWriter, NewWriter, Slicer,
    SlicerDirective,
};

use super::Row;

/// A reference to a `HMat` with arbitrarily ordered rows.
#[derive(Clone, Copy, Debug)]
pub struct HMatRef<'a, D, R> {
    pub(crate) row: &'a Row<D>,
    pub(crate) rem: R,
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

impl<'a, H, D, A> Slicer<'a, H, D, SlicerDirective<A, ()>> for HMatRef<'a, D, ()>
where
    H: AccessRowRef<D, A>,
{
    fn slice(h: &'a H) -> Self {
        HMatRef {
            row: h.get_row_ref(),
            rem: (),
        }
    }
}

impl<'a, H, D1, D2, R, A1, A2, Tail>
    Slicer<'a, H, D1, SlicerDirective<A1, SlicerDirective<A2, Tail>>>
    for HMatRef<'a, D1, HMatRef<'a, D2, R>>
where
    H: AccessRowRef<D1, A1>,
    H: AccessRowRef<D2, A2>,
    HMatRef<'a, D2, R>: Slicer<'a, H, D2, SlicerDirective<A2, Tail>>,
{
    fn slice(h: &'a H) -> Self {
        HMatRef {
            row: h.get_row_ref(),
            rem: <HMatRef<'a, D2, R> as Slicer<'a, H, D2, SlicerDirective<A2, Tail>>>::slice(h),
        }
    }
}

impl<'a, T1, T2, R> NewWriter for HMatRef<'a, T1, HMatRef<'a, T2, R>>
where
    HMatRef<'a, T2, R>: NewWriter,
{
    type Ret = HMatWriter<T1, <HMatRef<'a, T2, R> as NewWriter>::Ret>;

    fn new_writer(&self) -> Self::Ret {
        HMatWriter {
            row_mods: Default::default(),
            pd: PhantomData,
            rem: self.rem.new_writer(),
        }
    }
}

impl<'a, T1> NewWriter for HMatRef<'a, T1, ()> {
    type Ret = HMatWriter<T1, ()>;

    fn new_writer(&self) -> Self::Ret {
        HMatWriter {
            row_mods: Default::default(),
            pd: PhantomData,
            rem: (),
        }
    }
}
