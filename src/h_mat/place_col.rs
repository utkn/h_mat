use super::{HCol, HMat};

/// Represents a type that can store a column.
pub trait PlaceCol<T> {
    type Rem;
    /// Places the given column `col` at the given index `idx`.
    fn place_col(&mut self, idx: usize, col: HCol<T, Self::Rem>);
}

impl<T1, T2, R> PlaceCol<T1> for HMat<T1, HMat<T2, R>>
where
    HMat<T2, R>: PlaceCol<T2>,
{
    type Rem = HCol<T2, <HMat<T2, R> as PlaceCol<T2>>::Rem>;

    fn place_col(&mut self, idx: usize, col: HCol<T1, Self::Rem>) {
        if let Some(elem) = col.elem {
            self.head_row.place(idx, elem);
        }
        self.rem.place_col(idx, col.rem);
    }
}

impl<T> PlaceCol<T> for HMat<T, ()> {
    type Rem = ();

    fn place_col(&mut self, idx: usize, col: HCol<T, Self::Rem>) {
        if let Some(elem) = col.elem {
            self.head_row.place(idx, elem);
        }
    }
}
