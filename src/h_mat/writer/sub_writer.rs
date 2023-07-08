use std::marker::PhantomData;

use crate::HMatWriter;

/// Internal type used for the recursive implementations of the `GetSubWriter` trait.
pub struct GetSubWriterDirective<T>(PhantomData<*const T>);

/// Represents a writer type that can return one of its subwriters, e.g., `HMatWriter<T1, HMatWriter<T2, R>>` has a subwriter `HMatWriter<T2, R>`.
pub trait GetSubWriter<T, Directive> {
    type Rem;
    /// Returns the subwriter `HMatWriter<T, R>` as a mutable reference.
    fn sub_writer_mut(&mut self) -> &mut HMatWriter<T, Self::Rem>;
}

impl<D, R> GetSubWriter<D, ()> for HMatWriter<D, R> {
    type Rem = R;
    fn sub_writer_mut(&mut self) -> &mut HMatWriter<D, Self::Rem> {
        self
    }
}

impl<D, R, T, InnerDirective> GetSubWriter<D, GetSubWriterDirective<InnerDirective>>
    for HMatWriter<T, R>
where
    R: GetSubWriter<D, InnerDirective>,
{
    type Rem = <R as GetSubWriter<D, InnerDirective>>::Rem;
    fn sub_writer_mut(&mut self) -> &mut HMatWriter<D, Self::Rem> {
        self.rem.sub_writer_mut()
    }
}
