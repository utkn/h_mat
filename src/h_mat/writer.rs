use std::marker::PhantomData;

use itertools::Itertools;

use crate::{AccessRowMut, HMat, HMatRef, Row};

/// Represents a modification on a `Row<T>`.
pub enum RowMod<T> {
    SetCol(usize, T),
    UnsetCol(usize),
    UpdateCol(usize, Box<dyn FnOnce(&mut T)>),
}

impl<T> RowMod<T> {
    fn priority(&self) -> usize {
        match self {
            RowMod::SetCol(_, _) => 0,
            RowMod::UpdateCol(_, _) => 10,
            RowMod::UnsetCol(_) => 20,
        }
    }

    fn apply(self, row: &mut Row<T>) {
        match self {
            RowMod::SetCol(col_idx, new_val) => {
                row.place(col_idx, new_val);
            }
            RowMod::UnsetCol(col_idx) => {
                row.take(col_idx);
            }
            RowMod::UpdateCol(col_idx, f) => {
                row.get_mut(col_idx).map(|val| {
                    f(val);
                });
            }
        }
    }
}

/// A writer that can store a list of modifications, i.e., `RowMod`s that can be applied to a `HMat` in the future. Can be useful when it is not possible to maintain a mutable reference to the original matrix.
/// Note that the modifications are *NOT* applied in the same order they are appended to the writer. The order is always: `SetCol`, `UpdateCol`, and then `UnsetCol`.
pub struct HMatWriter<T, R> {
    row_mods: Vec<RowMod<T>>,
    rem: R,
    pd: PhantomData<*const T>,
}

/// Internal type used for the recursive implementations of the `GetWriter` trait.
pub struct GetWriterDirective<T>(PhantomData<*const T>);

pub trait GetWriter<T, R, Directive> {
    fn get_writer(&mut self) -> &mut HMatWriter<T, R>;
}

impl<D, R> GetWriter<D, R, ()> for HMatWriter<D, R> {
    fn get_writer(&mut self) -> &mut HMatWriter<D, R> {
        self
    }
}

impl<D, R, R1, T, InnerDirective> GetWriter<D, R, GetWriterDirective<InnerDirective>>
    for HMatWriter<T, R1>
where
    R1: GetWriter<D, R, InnerDirective>,
{
    fn get_writer(&mut self) -> &mut HMatWriter<D, R> {
        self.rem.get_writer()
    }
}

pub trait Writer<T> {
    fn set_col(&mut self, col_idx: usize, new_val: T);
    fn unset_col(&mut self, col_idx: usize);
    fn update_col(&mut self, col_idx: usize, f: impl FnOnce(&mut T) + 'static);
}

impl<T, R> Writer<T> for HMatWriter<T, R> {
    fn set_col(&mut self, col_idx: usize, new_val: T) {
        self.row_mods.push(RowMod::SetCol(col_idx, new_val));
    }

    fn unset_col(&mut self, col_idx: usize) {
        self.row_mods.push(RowMod::UnsetCol(col_idx));
    }

    fn update_col(&mut self, col_idx: usize, f: impl FnOnce(&mut T) + 'static) {
        self.row_mods.push(RowMod::UpdateCol(col_idx, Box::new(f)));
    }
}

/// Internal type used for the recursive implementations of the `ApplyWriter` trait.
pub struct ApplyWriterDirective<Head, Tail>(PhantomData<*const Head>, PhantomData<*const Tail>);

pub trait ApplyWriter<W, D> {
    fn apply(&mut self, w: W);
}

impl<D1, D2, R, A, Awt, Hh, Hr>
    ApplyWriter<HMatWriter<D1, HMatWriter<D2, R>>, ApplyWriterDirective<A, Awt>> for HMat<Hh, Hr>
where
    Self: AccessRowMut<D1, A>,
    Self: ApplyWriter<HMatWriter<D2, R>, Awt>,
{
    fn apply(&mut self, w: HMatWriter<D1, HMatWriter<D2, R>>) {
        let row_mut = self.get_row_mut();
        w.row_mods
            .into_iter()
            .sorted_by_key(|row_mod| row_mod.priority())
            .for_each(|row_mod| {
                row_mod.apply(row_mut);
            });
        self.apply(w.rem);
    }
}

impl<D, A, Hh, Hr> ApplyWriter<HMatWriter<D, ()>, ApplyWriterDirective<A, ()>> for HMat<Hh, Hr>
where
    Self: AccessRowMut<D, A>,
{
    fn apply(&mut self, w: HMatWriter<D, ()>) {
        let row_mut = self.get_row_mut();
        w.row_mods
            .into_iter()
            .sorted_by_key(|row_mod| row_mod.priority())
            .for_each(|row_mod| {
                row_mod.apply(row_mut);
            });
    }
}

/// Represents a type that can return a writer corresponding to its fields.
pub trait NewWriter {
    type Ret;
    /// Returns a new writer that can be used to gather modifications and apply them at once.
    fn new_writer(&self) -> Self::Ret;
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
