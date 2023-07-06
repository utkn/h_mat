mod h_mat;

pub use crate::h_mat::*;

#[cfg(test)]
mod tests {
    use crate::{Extend, HMatRef, Reform};

    use super::*;

    #[test]
    fn extend() {
        // Creating a HMat with i32, f32, usize rows.
        let _ = h_mat::HMat::new::<usize>().extend::<f32>().extend::<i32>();
    }

    #[test]
    fn access_row() {
        let mat = h_mat::HMat::new::<usize>().extend::<f32>().extend::<i32>();
        // Access the rows explicitly as a reference.
        let usize_row: &Row<usize> = mat.get_row_ref();
        let i32_row: &Row<i32> = mat.get_row_ref();
        // ... or as a mutable reference.
        let mut mat = mat;
        let i32_row_mut: &mut Row<i32> = mat.get_row_mut();
    }

    #[test]
    fn reform() {
        let mat = &&h_mat::HMat::new::<usize>().extend::<f32>().extend::<i32>();
        // Invoke `reform` to extract a reference matrix with arbitrary row order.
        // The returned type `HMatRef` is a heterogenous matrix of reference rows.
        let ref_mat: HMatRef<f32, HMatRef<i32, ()>> =
            Reform::<f32, ReformDirective<i32, _, _>, _>::reform(mat);
        // Access the rows as a reference to the original matrix.
        let f32_row: &Row<f32> = ref_mat.get_row_ref();
        let i32_row: &Row<i32> = ref_mat.get_row_ref();
    }

    #[test]
    fn reform_inference() {
        // We need to bind by && in order to be able to `reform`. This is because we utilize
        // the auto-deref specialization trick.
        // TODO: fix this
        let mat = &&h_mat::HMat::new::<usize>().extend::<f32>().extend::<i32>();
        // We call with && for auto type inference in let bindings.
        let _: HMatRef<f32, HMatRef<i32, ()>> = mat.reform();
        // ... also works as an argument!
        fn receive_reformed(_: HMatRef<f32, HMatRef<i32, ()>>) {}
        receive_reformed(mat.reform())
    }

    #[test]
    fn access_col() {
        let mat = h_mat::HMat::new::<usize>().extend::<f32>().extend::<i32>();
        // Access a single column as a reference.
        let col_ref: HCol<&i32, HCol<&f32, HCol<&usize, ()>>> = mat.get_col_ref(0);
        // ... or as a mutable reference...
        let mut mat = mat;
        let col_mut: HCol<&mut i32, HCol<&mut f32, HCol<&mut usize, ()>>> = mat.get_col_mut(0);
        // ... or directly move it out of the matrix.
        let col: HCol<i32, HCol<f32, HCol<usize, ()>>> = mat.take_col(0);
        // Then we can place it back to a different position.
        mat.place_col(1, col);
    }
}
