use serde::{Deserialize, Serialize};

mod sub_col;

use sub_col::*;

/// A single column of a `HMat`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HCol<T, R> {
    pub(crate) elem: Option<T>,
    pub(crate) rem: R,
}

impl<T, R> HCol<T, R> {
    fn get_first(&self) -> Option<&T> {
        self.elem.as_ref()
    }

    fn get_first_mut(&mut self) -> Option<&mut T> {
        self.elem.as_mut()
    }

    fn take_first(&mut self) -> Option<T> {
        self.elem.take()
    }

    fn place_first(&mut self, new_elem: T) -> Option<T> {
        self.elem.replace(new_elem)
    }

    /// Returns a reference to the element of type `D` in this column.
    pub fn get<'a, D, A>(&'a self) -> Option<&D>
    where
        Self: GetSubCol<'a, D, A>,
    {
        self.sub_col_ref().get_first()
    }

    /// Returns a mutable reference to the element of type `D` in this column.
    pub fn get_mut<'a, D, A>(&'a mut self) -> Option<&mut D>
    where
        Self: GetSubCol<'a, D, A>,
    {
        self.sub_col_mut().get_first_mut()
    }

    /// Removes and returns the element of type `D` in this column.
    pub fn take<'a, D, A>(&'a mut self) -> Option<D>
    where
        Self: GetSubCol<'a, D, A>,
    {
        self.sub_col_mut().take_first()
    }

    /// Places the given element `new_elem` on this column and returns the overwritten value.
    pub fn place<'a, D, A>(&'a mut self, new_elem: D) -> Option<D>
    where
        Self: GetSubCol<'a, D, A>,
    {
        self.sub_col_mut().place_first(new_elem)
    }
}
