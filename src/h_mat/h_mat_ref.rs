use crate::{AccessColRef, AccessRowRef, HCol, NewWriter};

use super::{HColIter, Row};

/// A reference to a `HMat` with arbitrarily ordered rows.
#[derive(Clone, Copy, Debug)]
pub struct HMatRef<'a, D, R> {
    pub(crate) row: &'a Row<D>,
    pub(crate) rem: R,
}

impl<'a, T, R> HMatRef<'a, T, R> {
    /// Returns a reference to the `Row<D>` of this matrix slice.
    pub fn get_row_ref<D, Directive>(&self) -> &Row<D>
    where
        Self: AccessRowRef<D, Directive>,
    {
        AccessRowRef::<D, Directive>::get_row_ref(self)
    }

    /// Returns a reference to the column at the given column index `col_idx`.
    pub fn get_col_ref(&'a self, col_idx: usize) -> HCol<&T, <Self as AccessColRef<'a, T>>::Rem>
    where
        Self: AccessColRef<'a, T>,
    {
        AccessColRef::<'a, T>::get_col_ref(self, col_idx)
    }

    /// Returns an iterator that iterates over `num_cols` many columns, with the column indexes from `0` to `num_cols-1`.
    pub fn iter(&'a self, num_cols: usize) -> HColIter<'a, Self>
    where
        Self: AccessColRef<'a, T>,
    {
        HColIter {
            mat_ref: self,
            curr_col_idx: 0,
            num_cols,
        }
    }

    /// Constructs a writer from this reference matrix.
    pub fn writer(&self) -> <Self as NewWriter>::Ret
    where
        Self: NewWriter,
    {
        NewWriter::new_writer(self)
    }
}
