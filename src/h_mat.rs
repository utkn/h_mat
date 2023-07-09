use serde::{Deserialize, Serialize};

mod access_col;
mod access_row;
mod extend;
mod h_col;
mod h_mat_ref;
mod iterator;
mod row;
mod slicer;
mod writer;

pub use access_col::*;
pub use access_row::*;
pub use extend::*;
pub use h_col::*;
pub use h_mat_ref::*;
pub use iterator::*;
pub use row::*;
pub use slicer::*;
pub use writer::*;

/// A heterogenous matrix, in which every row is a vector of a different type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HMat<T, R> {
    pub(crate) head_row: Row<T>,
    pub(crate) rem: R,
}

impl<T> HMat<T, ()> {
    /// Creates a new `HMat` with a single row of `T`.
    pub fn new() -> Self {
        HMat {
            head_row: Default::default(),
            rem: (),
        }
    }

    /// Creates a new `HMat` with a single row of `T` initialized with the given contents.
    pub fn new_with(iter: impl IntoIterator<Item = Option<T>>) -> Self {
        HMat {
            head_row: Row::from_iter(iter),
            rem: (),
        }
    }
}

impl<T, R> HMat<T, R> {
    /// Returns a reference to the `Row<D>` of this matrix.
    pub fn get_row_ref<D, Directive>(&self) -> &Row<D>
    where
        Self: AccessRowRef<D, Directive>,
    {
        AccessRowRef::<D, Directive>::get_row_ref(self)
    }

    /// Returns a mutable reference to the `Row<D>` of this matrix.
    pub fn get_row_mut<D, Directive>(&mut self) -> &mut Row<D>
    where
        Self: AccessRowMut<D, Directive>,
    {
        AccessRowMut::<D, Directive>::get_row_mut(self)
    }

    /// Returns a reference to the column at the given column index `col_idx`.
    pub fn get_col_ref<'a>(&'a self, col_idx: usize) -> HCol<&T, <Self as AccessColRef<'a, T>>::Rem>
    where
        Self: AccessColRef<'a, T>,
    {
        AccessColRef::<'a, T>::get_col_ref(self, col_idx)
    }

    /// Returns a mutable reference to the column at the given column index `col_idx`.
    pub fn get_col_mut<'a>(
        &'a mut self,
        col_idx: usize,
    ) -> HCol<&mut T, <Self as AccessColMut<'a, T>>::Rem>
    where
        Self: AccessColMut<'a, T>,
    {
        AccessColMut::<'a, T>::get_col_mut(self, col_idx)
    }

    /// Removes and returns the column at the given column index `col_idx`.
    pub fn take_col(&mut self, col_idx: usize) -> HCol<T, <Self as TakeCol<T>>::Rem>
    where
        Self: TakeCol<T>,
    {
        TakeCol::<T>::take_col(self, col_idx)
    }

    /// Places the given column `col` at the given column index `col_idx`.
    pub fn place_col(&mut self, col_idx: usize, col: HCol<T, <Self as PlaceCol<T>>::Rem>)
    where
        Self: PlaceCol<T>,
    {
        PlaceCol::<T>::place_col(self, col_idx, col)
    }

    /// Returns a *slice*, i.e., a subset of the rows, of this matrix.
    pub fn slice<'a, D, Sr, Directive>(&'a self) -> HMatRef<'a, D, Sr>
    where
        HMatRef<'a, D, Sr>: Slicer<'a, Self, D, Directive>,
    {
        HMatRef::slice(self)
    }

    /// Returns an iterator that iterates over `num_cols` many columns, with the column indexes from `0` to `num_cols-1`.
    pub fn iter<'a>(&'a self, num_cols: usize) -> HColIter<'a, Self>
    where
        Self: AccessColRef<'a, T>,
    {
        HColIter {
            mat_ref: self,
            curr_col_idx: 0,
            num_cols,
        }
    }

    /// Modifies the matrix with the modifications stored in the given `HMatWriter`.
    pub fn write_with<T1, R1>(&mut self, w: HMatWriter<T1, R1>)
    where
        Self: ApplyWriter<HMatWriter<T1, R1>, T1>,
    {
        ApplyWriter::<HMatWriter<T1, R1>, T1>::apply(self, w)
    }
}
