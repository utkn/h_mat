use crate::{AccessColRef, HCol, HMatRef};

/// An iterator that iterates over the columns of `H`.
pub struct HColIter<'a, H> {
    mat_ref: &'a H,
    curr_col_idx: usize,
    num_cols: usize,
}

impl<'a, T, R> Iterator for HColIter<'a, HMatRef<'a, T, R>>
where
    HMatRef<'a, T, R>: AccessColRef<'a, T>,
{
    type Item = HCol<&'a T, <HMatRef<'a, T, R> as AccessColRef<'a, T>>::Rem>;

    fn next(&mut self) -> Option<Self::Item> {
        let col_idx = self.curr_col_idx;
        if col_idx == self.num_cols {
            return None;
        }
        self.curr_col_idx += 1;
        let col = self.mat_ref.get_col_ref(col_idx);
        Some(col)
    }
}

impl<'a, T, R> HMatRef<'a, T, R> {
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
}
