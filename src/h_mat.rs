use serde::{Deserialize, Serialize};

mod access_col;
mod access_row;
mod h_col;
mod h_mat_ref;
mod place_col;
mod reform;
mod row;

pub use access_col::*;
pub use access_row::*;
pub use h_col::*;
pub use h_mat_ref::*;
pub use place_col::*;
pub use reform::*;
pub use row::*;

/// A heterogenous matrix, in which every row is a vector of a different type.
/// For example, `HMat<Position, HMat<Velocity, ()>>` is a 2xN matrix, in which the first row is a `Vec<Option<Position>>`, and the second row is a `Vec<Option<Velocity>>`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HMat<T, R> {
    row: Row<T>,
    rem: R,
}

pub trait Extend {
    type Old;
    /// Extends this `HMat<T, _>` with a new row of type `E`, returning `HMat<E, HMat<T, _>>`.
    fn extend<E>(self) -> HMat<E, Self::Old>;
}

impl<T, R> Extend for HMat<T, R> {
    type Old = HMat<T, R>;
    fn extend<E>(self) -> HMat<E, Self::Old> {
        HMat {
            row: Default::default(),
            rem: self,
        }
    }
}

impl HMat<(), ()> {
    /// Creates a new `HMat` with a single row of `T`.
    pub fn new<T>() -> HMat<T, ()> {
        HMat {
            row: Default::default(),
            rem: (),
        }
    }
}

// Implementation of AccessRowRef for HMat
impl<D, R> AccessRowRef<D, ()> for HMat<D, R> {
    fn get_row_ref(&self) -> &Row<D> {
        &self.row
    }
}

impl<T, R, D, A> AccessRowRef<D, AccessRowDirective<A>> for HMat<T, R>
where
    R: AccessRowRef<D, A>,
{
    fn get_row_ref(&self) -> &Row<D> {
        self.rem.get_row_ref()
    }
}

// Implementation of AccessRowMut for HMat
impl<D, R> AccessRowMut<D, ()> for HMat<D, R> {
    fn get_row_mut(&mut self) -> &mut Row<D> {
        &mut self.row
    }
}

impl<T, R, D, A> AccessRowMut<D, AccessRowDirective<A>> for HMat<T, R>
where
    R: AccessRowMut<D, A>,
{
    fn get_row_mut(&mut self) -> &mut Row<D> {
        self.rem.get_row_mut()
    }
}

impl<'a, T1, T2, R> AccessColRef<'a, T1> for HMat<T1, HMat<T2, R>>
where
    Self: 'a,
    HMat<T2, R>: AccessColRef<'a, T2>,
{
    type Rem = HCol<&'a T2, <HMat<T2, R> as AccessColRef<'a, T2>>::Rem>;
    fn get_col_ref(&'a self, idx: usize) -> HCol<&T1, Self::Rem> {
        HCol {
            elem: self.row.get(idx),
            rem: self.rem.get_col_ref(idx),
        }
    }
}

impl<'a, T> AccessColRef<'a, T> for HMat<T, ()> {
    type Rem = ();
    fn get_col_ref(&'a self, idx: usize) -> HCol<&T, Self::Rem> {
        HCol {
            elem: self.row.get(idx),
            rem: (),
        }
    }
}

impl<'a, T1, T2, R> AccessColMut<'a, T1> for HMat<T1, HMat<T2, R>>
where
    Self: 'a,
    HMat<T2, R>: AccessColMut<'a, T2>,
{
    type Rem = HCol<&'a mut T2, <HMat<T2, R> as AccessColMut<'a, T2>>::Rem>;
    fn get_col_mut(&'a mut self, idx: usize) -> HCol<&mut T1, Self::Rem> {
        HCol {
            elem: self.row.get_mut(idx),
            rem: self.rem.get_col_mut(idx),
        }
    }
}

impl<'a, T> AccessColMut<'a, T> for HMat<T, ()> {
    type Rem = ();
    fn get_col_mut(&mut self, idx: usize) -> HCol<&mut T, Self::Rem> {
        HCol {
            elem: self.row.get_mut(idx),
            rem: (),
        }
    }
}

impl<'a, T1, T2, R> TakeCol<'a, T1> for HMat<T1, HMat<T2, R>>
where
    Self: 'a,
    HMat<T2, R>: TakeCol<'a, T2>,
{
    type Rem = HCol<T2, <HMat<T2, R> as TakeCol<'a, T2>>::Rem>;
    fn take_col(&mut self, idx: usize) -> HCol<T1, Self::Rem> {
        HCol {
            elem: self.row.take(idx),
            rem: self.rem.take_col(idx),
        }
    }
}
impl<'a, T> TakeCol<'a, T> for HMat<T, ()> {
    type Rem = ();
    fn take_col(&mut self, idx: usize) -> HCol<T, Self::Rem> {
        HCol {
            elem: self.row.take(idx),
            rem: (),
        }
    }
}
