use serde::{Deserialize, Serialize};

mod access_col;
mod access_row;
mod h_col;
mod h_mat_ref;
mod place_col;
mod row;

pub use access_col::*;
pub use access_row::*;
pub use h_col::*;
pub use h_mat_ref::*;
pub use place_col::*;
pub use row::*;

/// A heterogenous matrix, in which every row is a vector of a different type.
/// For example, `HMat<Position, HMat<Velocity, ()>>` is a 2xN matrix, in which the first row is a `Vec<Option<Position>>`, and the second row is a `Vec<Option<Velocity>>`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HMat<T, R> {
    row: Row<T>,
    rem: R,
}

pub trait Extend: Sized {
    /// Extends this `HMat<T, _>` with a new row of type `E`, returning `HMat<E, HMat<T, _>>`.
    fn extend<E>(self) -> HMat<E, Self>;
}

impl<T, R> Extend for HMat<T, R> {
    fn extend<E>(self) -> HMat<E, Self> {
        HMat {
            row: Default::default(),
            rem: self,
        }
    }
}

impl<T> HMat<T, ()> {
    /// Creates a new `HMat` with a single row of `T`.
    pub fn new() -> Self {
        HMat {
            row: Default::default(),
            rem: (),
        }
    }
}
