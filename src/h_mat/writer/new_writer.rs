use std::marker::PhantomData;

use crate::{HMat, HMatRef, HMatWriter};

/// Represents a type that can return a writer corresponding to its fields.
pub trait NewWriter<T> {
    type Rem;
    /// Returns a new writer that can be used to gather modifications and apply them at once.
    fn new_writer(&self) -> HMatWriter<T, Self::Rem>;
}

impl<'a, T1, T2, R> NewWriter<T1> for HMatRef<'a, T1, HMatRef<'a, T2, R>>
where
    HMatRef<'a, T2, R>: NewWriter<T2>,
{
    type Rem = HMatWriter<T2, <HMatRef<'a, T2, R> as NewWriter<T2>>::Rem>;

    fn new_writer(&self) -> HMatWriter<T1, Self::Rem> {
        HMatWriter {
            row_mods: Default::default(),
            pd: PhantomData,
            rem: self.rem.new_writer(),
        }
    }
}

impl<'a, T1> NewWriter<T1> for HMatRef<'a, T1, ()> {
    type Rem = ();

    fn new_writer(&self) -> HMatWriter<T1, Self::Rem> {
        HMatWriter {
            row_mods: Default::default(),
            pd: PhantomData,
            rem: (),
        }
    }
}

impl<T1, T2, R> NewWriter<T1> for HMat<T1, HMat<T2, R>>
where
    HMat<T2, R>: NewWriter<T2>,
{
    type Rem = HMatWriter<T2, <HMat<T2, R> as NewWriter<T2>>::Rem>;

    fn new_writer(&self) -> HMatWriter<T1, Self::Rem> {
        HMatWriter {
            row_mods: Default::default(),
            pd: PhantomData,
            rem: self.rem.new_writer(),
        }
    }
}

impl<T1> NewWriter<T1> for HMat<T1, ()> {
    type Rem = ();

    fn new_writer(&self) -> HMatWriter<T1, Self::Rem> {
        HMatWriter {
            row_mods: Default::default(),
            pd: PhantomData,
            rem: (),
        }
    }
}
