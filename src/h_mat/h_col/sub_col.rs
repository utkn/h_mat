use std::marker::PhantomData;

use crate::HCol;

/// Internal type used for the recursive implementations of the `GetSubCol` trait.
pub struct GetSubColDirective<T>(PhantomData<T>);

/// Represents a column that can return one of its subcolumns, e.g., `HMatCol<T1, HMatCol<T2, R>>` has a subcolumn `HMatCol<T2, R>`.
pub trait GetSubCol<'a, D, Directive> {
    type Rem: 'a;
    /// Returns the subcolumn `HCol<D, R>` as a reference.
    fn sub_col_ref(&self) -> &HCol<D, Self::Rem>;
    /// Returns the subcolumn `HCol<D, R>` as a mutable reference.
    fn sub_col_mut(&mut self) -> &mut HCol<D, Self::Rem>;
}

impl<'a, D, R> GetSubCol<'a, D, ()> for HCol<D, R>
where
    R: 'a,
{
    type Rem = R;
    fn sub_col_ref(&self) -> &HCol<D, Self::Rem> {
        self
    }

    fn sub_col_mut(&mut self) -> &mut HCol<D, Self::Rem> {
        self
    }
}

impl<'a, D, T, R, InnerDirective> GetSubCol<'a, D, GetSubColDirective<InnerDirective>>
    for HCol<T, R>
where
    R: GetSubCol<'a, D, InnerDirective>,
{
    type Rem = <R as GetSubCol<'a, D, InnerDirective>>::Rem;
    fn sub_col_ref(&self) -> &HCol<D, Self::Rem> {
        self.rem.sub_col_ref()
    }

    fn sub_col_mut(&mut self) -> &mut HCol<D, Self::Rem> {
        self.rem.sub_col_mut()
    }
}
