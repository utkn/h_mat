use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

/// A single column of a `HMat`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HCol<T, R> {
    pub(crate) elem: Option<T>,
    pub(crate) rem: R,
}

impl<T, R> HCol<T, R> {
    pub fn get(&self) -> Option<&T> {
        self.elem.as_ref()
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.elem.as_mut()
    }

    pub fn take(&mut self) -> Option<T> {
        self.elem.take()
    }

    pub fn place(&mut self, new_elem: T) -> Option<T> {
        self.elem.replace(new_elem)
    }
}

/// Internal type used for the recursive implementations of the `GetSubCol` trait.
pub struct GetSubColDirective<T>(PhantomData<T>);

/// Represents a column that can return one of its subcolumns, e.g., `HMatCol<T1, HMatCol<T2, R>>` has a subcolumn `HMatCol<T2, R>`.
pub trait GetSubCol<D, R, Directive> {
    /// Returns the subcolumn `HCol<D, R>` as a reference.
    fn subcol_ref(&self) -> &HCol<D, R>;
    /// Returns the subcolumn `HCol<D, R>` as a mutable reference.
    fn subcol_mut(&mut self) -> &mut HCol<D, R>;
}

impl<D, R> GetSubCol<D, R, ()> for HCol<D, R> {
    fn subcol_ref(&self) -> &HCol<D, R> {
        self
    }

    fn subcol_mut(&mut self) -> &mut HCol<D, R> {
        self
    }
}

impl<D, R, T, R1, InnerDirective> GetSubCol<D, R, GetSubColDirective<InnerDirective>>
    for HCol<T, R1>
where
    R1: GetSubCol<D, R, InnerDirective>,
{
    fn subcol_ref(&self) -> &HCol<D, R> {
        self.rem.subcol_ref()
    }

    fn subcol_mut(&mut self) -> &mut HCol<D, R> {
        self.rem.subcol_mut()
    }
}
