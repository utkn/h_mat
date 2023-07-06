use super::{HCol, HMat};

pub trait AccessColRef<'a, T> {
    type Rem;
    fn get_col_ref(&'a self, idx: usize) -> HCol<&T, Self::Rem>;
}

pub trait AccessColMut<'a, T> {
    type Rem;
    fn get_col_mut(&'a mut self, idx: usize) -> HCol<&mut T, Self::Rem>;
}

pub trait TakeCol<'a, T> {
    type Rem;
    fn take_col(&mut self, idx: usize) -> HCol<T, Self::Rem>;
}
