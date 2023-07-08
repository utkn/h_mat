use std::marker::PhantomData;

pub struct ReformDirective<Head, Tail, Access>(
    PhantomData<*const Head>,
    PhantomData<*const Tail>,
    PhantomData<*const Access>,
);

/// Represents a type that can construct itself by rearranging the fields of the original type `H`.
pub trait Reformer<'a, H, Head, Directive, Access> {
    fn reform(h: &'a H) -> Self;
}
