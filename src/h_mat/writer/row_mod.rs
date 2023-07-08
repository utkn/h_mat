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
