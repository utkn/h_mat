use super::HCol;

pub trait AccessColRef<'a, T> {
    type Rem;
    /// Returns the column with the given index `idx` as a reference.
    fn get_col_ref(&'a self, idx: usize) -> HCol<&T, Self::Rem>;
}

pub trait AccessColMut<'a, T> {
    type Rem;
    /// Returns the column with the given index `idx` as a mutable reference.
    fn get_col_mut(&'a mut self, idx: usize) -> HCol<&mut T, Self::Rem>;
}

pub trait TakeCol<'a, T> {
    type Rem;
    /// Removes the column with the given index `idx` and returns it. The corresponding values in the original matrix will be set to `None` after this operation.
    fn take_col(&mut self, idx: usize) -> HCol<T, Self::Rem>;
}
