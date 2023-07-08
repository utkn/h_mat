use std::marker::PhantomData;

use crate::Row;

/// Represents a modification on a `Row<T>`.
pub enum RowMod<T> {
    SetCol(usize, T),
    UnsetCol(usize),
    UpdateCol(usize, Box<dyn FnOnce(&mut T)>),
}

impl<T> RowMod<T> {
    pub(crate) fn priority(&self) -> usize {
        match self {
            RowMod::SetCol(_, _) => 0,
            RowMod::UpdateCol(_, _) => 10,
            RowMod::UnsetCol(_) => 20,
        }
    }

    pub(crate) fn apply(self, row: &mut Row<T>) {
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
    pub(crate) row_mods: Vec<RowMod<T>>,
    pub(crate) rem: R,
    pub(crate) pd: PhantomData<*const T>,
}

/// Internal type used for the recursive implementations of the `GetSubWriter` trait.
pub struct GetSubWriterDirective<T>(PhantomData<*const T>);

/// Represents a writer type that can return one of its subwriters, e.g., `HMatWriter<T1, HMatWriter<T2, R>>` has a subwriter `HMatWriter<T2, R>`.
pub trait GetSubWriter<T, R, Directive> {
    fn get_writer(&mut self) -> &mut HMatWriter<T, R>;
}

impl<D, R> GetSubWriter<D, R, ()> for HMatWriter<D, R> {
    fn get_writer(&mut self) -> &mut HMatWriter<D, R> {
        self
    }
}

impl<D, R, R1, T, InnerDirective> GetSubWriter<D, R, GetSubWriterDirective<InnerDirective>>
    for HMatWriter<T, R1>
where
    R1: GetSubWriter<D, R, InnerDirective>,
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

/// Represents a type that can receive a writer `W` to modify itself.
pub trait ApplyWriter<W, D> {
    fn apply(&mut self, w: W);
}

/// Represents a type that can return a writer corresponding to its fields.
pub trait NewWriter {
    type Ret;
    /// Returns a new writer that can be used to gather modifications and apply them at once.
    fn new_writer(&self) -> Self::Ret;
}
