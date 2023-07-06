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

// Receiver T for lower priority when called with (&&T).
impl<'a, D, A, T> Reform<'a, D, (), A> for T
where
    D: 'static,
    T: AccessRowRef<D, A>,
{
    type Rem = ();

    fn reform(&'a self) -> HMatRef<'a, D, Self::Rem> {
        HMatRef {
            row: (self).get_row_ref(),
            rem: (),
        }
    }
}

// Receiver &T for higher priority when called with (&&T).
impl<'a, D1, D2, A1, A2, R2, T> Reform<'a, D1, ReformDirective<D2, R2, A2>, A1> for &T
where
    D1: 'static,
    D2: 'static,
    T: AccessRowRef<D1, A1> + Reform<'a, D2, R2, A2> + 'a,
{
    type Rem = HMatRef<'a, D2, <T as Reform<'a, D2, R2, A2>>::Rem>;

    fn reform(&'a self) -> HMatRef<'a, D1, Self::Rem> {
        HMatRef {
            row: self.get_row_ref(),
            rem: (*self).reform(),
        }
    }
}
