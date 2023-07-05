use super::{HMat, Row};

#[derive(Clone, Copy, Debug)]
pub struct AccessRowDirective<T>(T);

pub trait AccessRow<D, Directive> {
    fn get_row_ref(&self) -> &Row<D>;
    fn get_row_mut(&mut self) -> &mut Row<D>;
}

impl<D, R> AccessRow<D, ()> for HMat<D, R> {
    fn get_row_ref(&self) -> &Row<D> {
        &self.row
    }

    fn get_row_mut(&mut self) -> &mut Row<D> {
        &mut self.row
    }
}

impl<T, R, D, A> AccessRow<D, AccessRowDirective<A>> for HMat<T, R>
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
