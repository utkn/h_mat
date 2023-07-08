mod h_mat;

pub use crate::h_mat::*;

#[cfg(test)]
mod tests {
    use crate::{Extend, HMat, HMatRef, Row};

    #[test]
    fn integration() {
        let mat = HMat::<usize, ()>::new().extend::<f32>().extend::<i32>();
        {
            // Access the rows explicitly as a reference.
            let usize_row = mat.get_row_ref::<usize, _>();
            let i32_row = mat.get_row_ref::<i32, _>();
            assert_eq!(usize_row, &Row::<usize>::default());
            assert_eq!(i32_row, &Row::<i32>::default());
        }
        let mat_ref: HMatRef<f32, HMatRef<i32, ()>> = mat.slice();
        mat_ref.get_row_ref::<f32, _>();
    }
}
