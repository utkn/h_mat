use crate::{AccessColRef, HCol, HMatRef};

/// An iterator that iterates over the columns of `H`.
pub struct HColIter<'a, H> {
    pub(crate) mat_ref: &'a H,
    pub(crate) curr_col_idx: usize,
    pub(crate) num_cols: usize,
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
