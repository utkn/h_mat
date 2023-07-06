use crate::{AccessRowDirective, AccessRowRef};

use super::Row;

#[derive(Clone, Copy, Debug)]
pub struct HMatRef<'a, D, R> {
    pub row: &'a Row<D>,
    pub rem: R,
}

// Implementation of AccessRowRef for HMatRef
impl<'a, D, R> AccessRowRef<D, ()> for HMatRef<'a, D, R> {
    fn get_row_ref(&self) -> &Row<D> {
        &self.row
    }
}

impl<'a, T, R, D, A> AccessRowRef<D, AccessRowDirective<A>> for HMatRef<'a, T, R>
where
    R: AccessRowRef<D, A>,
{
    fn get_row_ref(&self) -> &Row<D> {
        self.rem.get_row_ref()
    }
}
