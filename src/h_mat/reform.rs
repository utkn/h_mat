use std::marker::PhantomData;

use super::HMatRef;

pub struct ReformDirective<Head, Tail, Access>(
    PhantomData<*const Head>,
    PhantomData<*const Tail>,
    PhantomData<*const Access>,
);

/// Represents a type that can be converted into a `HMatRef` with arbitrary row order.
pub trait Reform<'a, Head, Directive, Access> {
    type Rem;
    fn reform(&'a self) -> HMatRef<'a, Head, Self::Rem>;
}
