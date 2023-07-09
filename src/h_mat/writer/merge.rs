use std::marker::PhantomData;

use crate::{h_mat::writer::sub_writer::GetSubWriter, HMatWriter};

/// Represents a recursive type that can be merged with another recursive type.
pub trait Merge<Other, Directive> {
    /// Merges this with `other` recursively.
    fn merge(&mut self, other: Other);
}

/// Internal type used for the recursive implementations of the `Merge` trait.
pub struct MergeDirective<Head, Tail>(PhantomData<Head>, PhantomData<Tail>);

impl<T, D, R, A1, A2, DirectiveTail>
    Merge<HMatWriter<D, R>, MergeDirective<A1, MergeDirective<A2, DirectiveTail>>> for T
where
    T: GetSubWriter<D, A1> + Merge<R, MergeDirective<A2, DirectiveTail>>,
{
    fn merge(&mut self, other: HMatWriter<D, R>) {
        self.sub_writer_mut()
            .row_mods
            .extend(other.row_mods.into_iter());
        self.merge(other.rem);
    }
}

impl<T, D, A> Merge<HMatWriter<D, ()>, MergeDirective<A, ()>> for T
where
    T: GetSubWriter<D, A>,
{
    fn merge(&mut self, other: HMatWriter<D, ()>) {
        self.sub_writer_mut()
            .row_mods
            .extend(other.row_mods.into_iter())
    }
}
