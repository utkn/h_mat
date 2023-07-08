use std::marker::PhantomData;

use crate::{HMatRef, HMatWriter};

/// Represents a type that can return a writer corresponding to its fields.
pub trait NewWriter {
    type Ret;
    /// Returns a new writer that can be used to gather modifications and apply them at once.
    fn new_writer(&self) -> Self::Ret;
}

impl<'a, T1, T2, R> NewWriter for HMatRef<'a, T1, HMatRef<'a, T2, R>>
where
    HMatRef<'a, T2, R>: NewWriter,
{
    type Ret = HMatWriter<T1, <HMatRef<'a, T2, R> as NewWriter>::Ret>;

    fn new_writer(&self) -> Self::Ret {
        HMatWriter {
            row_mods: Default::default(),
            pd: PhantomData,
            rem: self.rem.new_writer(),
        }
    }
}

impl<'a, T1> NewWriter for HMatRef<'a, T1, ()> {
    type Ret = HMatWriter<T1, ()>;

    fn new_writer(&self) -> Self::Ret {
        HMatWriter {
            row_mods: Default::default(),
            pd: PhantomData,
            rem: (),
        }
    }
}
