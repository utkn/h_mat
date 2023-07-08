use std::marker::PhantomData;

/// Internal type used for the recursive implementations of the `Slicer` trait.
#[derive(Clone, Copy, Debug)]
pub struct SlicerDirective<Head, Tail>(PhantomData<*const Head>, PhantomData<*const Tail>);

/// Represents a type that can construct itself by rearranging the fields of the original type `H`.
pub trait Slicer<'a, H, D, Directive> {
    fn slice(h: &'a H) -> Self;
}
