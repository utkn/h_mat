use std::marker::PhantomData;

/// Internal type used for the recursive implementations of the `Reformer` trait.
#[derive(Clone, Copy, Debug)]
pub struct ReformerDirective<Head, Tail>(PhantomData<*const Head>, PhantomData<*const Tail>);

/// Represents a type that can construct itself by rearranging the fields of the original type `H`.
pub trait Reformer<'a, H, D, Directive> {
    fn reform(h: &'a H) -> Self;
}
