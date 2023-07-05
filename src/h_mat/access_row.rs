use super::{HMat, Row};

#[derive(Clone, Copy, Debug)]
pub struct Direct;

#[derive(Clone, Copy, Debug)]
pub struct Indirect<T>(T);

pub trait AccessRow<D, A> {
    fn get_row_ref(&self) -> &Row<D>;
    fn get_row_mut(&mut self) -> &mut Row<D>;
}

impl<D, R> AccessRow<D, Direct> for HMat<D, R> {
    fn get_row_ref(&self) -> &Row<D> {
        &self.row
    }

    fn get_row_mut(&mut self) -> &mut Row<D> {
        &mut self.row
    }
}

impl<T, R, D, A> AccessRow<D, Indirect<A>> for HMat<T, R>
where
    R: AccessRow<D, A>,
{
    fn get_row_ref(&self) -> &Row<D> {
        self.rem.get_row_ref()
    }

    fn get_row_mut(&mut self) -> &mut Row<D> {
        self.rem.get_row_mut()
    }
}
