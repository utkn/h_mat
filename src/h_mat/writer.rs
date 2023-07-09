use std::marker::PhantomData;

use itertools::Itertools;

use crate::{AccessRowMut, HMat};

mod merge;
mod new_writer;
mod row_mod;
mod sub_writer;

pub use merge::*;
pub use new_writer::*;
pub use row_mod::*;
pub use sub_writer::*;

/// A writer that can store a list of modifications, i.e., `RowMod`s that can be applied to a `HMat` in the future. Can be useful when it is not possible to maintain a mutable reference to the original matrix.
/// Note that the modifications are **NOT** applied in the same order they are appended to the writer. The order is always: `SetCol`, `UpdateCol`, and then `UnsetCol`.
pub struct HMatWriter<T, R> {
    pub(crate) row_mods: Vec<RowMod<T>>,
    pub(crate) rem: R,
    pub(crate) pd: PhantomData<*const T>,
}

/// Internal type used for the recursive implementations of the `ApplyWriter` trait.
pub struct ApplyWriterDirective<Head, Tail>(PhantomData<*const Head>, PhantomData<*const Tail>);

/// Represents a type that can receive a writer `W` to modify itself.
pub trait ApplyWriter<W, Directive> {
    fn apply(&mut self, w: W);
}

impl<D1, D2, R, A, Awt, Hh, Hr>
    ApplyWriter<HMatWriter<D1, HMatWriter<D2, R>>, ApplyWriterDirective<A, Awt>> for HMat<Hh, Hr>
where
    Self: AccessRowMut<D1, A>,
    Self: ApplyWriter<HMatWriter<D2, R>, Awt>,
{
    fn apply(&mut self, w: HMatWriter<D1, HMatWriter<D2, R>>) {
        let row_mut = self.get_row_mut();
        w.row_mods
            .into_iter()
            .sorted_by_key(|row_mod| row_mod.priority())
            .for_each(|row_mod| {
                row_mod.apply(row_mut);
            });
        self.apply(w.rem);
    }
}

impl<D, A, Hh, Hr> ApplyWriter<HMatWriter<D, ()>, ApplyWriterDirective<A, ()>> for HMat<Hh, Hr>
where
    Self: AccessRowMut<D, A>,
{
    fn apply(&mut self, w: HMatWriter<D, ()>) {
        let row_mut = self.get_row_mut();
        w.row_mods
            .into_iter()
            .sorted_by_key(|row_mod| row_mod.priority())
            .for_each(|row_mod| {
                row_mod.apply(row_mut);
            });
    }
}

impl<T, R> HMatWriter<T, R> {
    pub fn set_col<D, A>(&mut self, col_idx: usize, new_val: D)
    where
        Self: GetSubWriter<D, A>,
    {
        self.sub_writer_mut()
            .row_mods
            .push(RowMod::SetCol(col_idx, new_val));
    }

    pub fn unset_col<D, A>(&mut self, col_idx: usize)
    where
        Self: GetSubWriter<D, A>,
    {
        self.sub_writer_mut()
            .row_mods
            .push(RowMod::UnsetCol(col_idx));
    }

    pub fn update_col<D, A>(&mut self, col_idx: usize, f: impl FnOnce(&mut D) + 'static)
    where
        Self: GetSubWriter<D, A>,
    {
        self.sub_writer_mut()
            .row_mods
            .push(RowMod::UpdateCol(col_idx, Box::new(f)));
    }

    pub fn merge<T2, R2, Directive>(&mut self, other: HMatWriter<T2, R2>)
    where
        Self: Merge<HMatWriter<T2, R2>, Directive>,
    {
        Merge::<HMatWriter<T2, R2>, Directive>::merge(self, other);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    use super::new_writer::NewWriter;

    #[test]
    fn basic() {
        let mut mat = HMat::<usize, ()>::new().extend::<f32>().extend::<i32>();
        {
            let ref_mat: HMatRef<f32, HMatRef<i32, ()>> = HMatRef::slice(&mat);
            let mut writer = ref_mat.new_writer();
            // Set the column 0 of the i32 row.
            writer.set_col(0, 3);
            // Update the column 0 of the i32 row.
            writer.update_col(0, |val: &mut i32| {
                *val += 1;
            });
            // Apply the modifications.
            mat.apply(writer);
        }
        {
            let ref_mat: HMatRef<f32, HMatRef<i32, ()>> = HMatRef::slice(&mat);
            assert_eq!(ref_mat.get_row_ref(), &Row::<i32>::from_iter([Some(4)]));
        }
        {
            let ref_mat: HMatRef<f32, HMatRef<i32, ()>> = HMatRef::slice(&mat);
            let mut writer = ref_mat.new_writer();
            // Remove the column 0 of the i32 row.
            writer.unset_col::<i32, _>(0);
            mat.apply(writer);
        }
        {
            let ref_mat: HMatRef<f32, HMatRef<i32, ()>> = HMatRef::slice(&mat);
            assert_eq!(ref_mat.get_row_ref(), &Row::<i32>::from_iter([None]));
        }
    }

    #[test]
    fn merge() {
        let mut mat = HMat::<usize, ()>::new().extend::<f32>().extend::<i32>();
        let w1 = {
            let ref_mat: HMatRef<f32, HMatRef<i32, ()>> = HMatRef::slice(&mat);
            let mut writer = ref_mat.new_writer();
            // Set the column 0 of the i32 row.
            writer.set_col(0, 3);
            // Update the column 0 of the i32 row.
            writer.update_col(0, |val: &mut i32| {
                *val += 1;
            });
            writer
        };
        let mut w2 = {
            let mut writer = mat.new_writer();
            // Update the column 0 of the i32 row.
            writer.update_col(0, |val: &mut i32| {
                *val += 1;
            });
            writer
        };
        w2.merge(w1);
        mat.apply(w2);
        {
            let ref_mat: HMatRef<i32, ()> = HMatRef::slice(&mat);
            assert_eq!(ref_mat.get_row_ref(), &Row::<i32>::from_iter([Some(5)]));
        }
    }
}
