mod h_mat;

pub use crate::h_mat::*;

#[cfg(test)]
mod tests {
    use crate::{Extend, HMatRef};

    use super::*;

    #[test]
    fn extend() {
        // Creating a HMat with i32, f32, usize rows.
        let mat = h_mat::HMat::new::<usize>().extend::<f32>().extend::<i32>();
        assert_eq!(mat.head_row, Row::<i32>::default());
        assert_eq!(mat.rem.head_row, Row::<f32>::default());
        assert_eq!(mat.rem.rem.head_row, Row::<usize>::default());
        assert_eq!(mat.rem.rem.rem, ());
    }

    #[test]
    fn access_row() {
        let mut mat = h_mat::HMat::new::<usize>().extend::<f32>().extend::<i32>();
        {
            // Access the rows explicitly as a reference.
            let usize_row: &Row<usize> = mat.get_row_ref();
            let i32_row: &Row<i32> = mat.get_row_ref();
            assert_eq!(usize_row, &Row::<usize>::default());
            assert_eq!(i32_row, &Row::<i32>::default());
        }
        {
            // ... or as a mutable reference.
            let i32_row_mut: &mut Row<i32> = mat.get_row_mut();
            assert_eq!(i32_row_mut.place(0, 1), None);
            assert_eq!(i32_row_mut.place(2, 3), None);
            assert_eq!(
                i32_row_mut,
                &Row::<i32>::from_iter([Some(1), None, Some(3)])
            );
            i32_row_mut.get_mut(2).map(|val| *val += 1);
            assert_eq!(
                i32_row_mut,
                &Row::<i32>::from_iter([Some(1), None, Some(4)])
            );
            assert_eq!(i32_row_mut.take(0), Some(1));
            assert_eq!(i32_row_mut, &Row::from_iter([None, None, Some(4)]));
        }
        {
            let i32_row: &Row<i32> = mat.get_row_ref();
            assert_eq!(i32_row, &Row::from_iter([None, None, Some(4)]));
        }
    }

    #[test]
    fn slice() {
        let mat = h_mat::HMat::new_with::<usize>([Some(0), Some(1), Some(2)])
            .extend_with::<f32, _>([None, Some(0.5), None])
            .extend_with::<i32, _>([None, None, Some(-1)]);
        // Invoke `slice` to extract a reference matrix with arbitrary row order.
        // The returned type `HMatRef` is a heterogenous matrix of reference rows.
        let ref_mat: HMatRef<f32, HMatRef<i32, ()>> = HMatRef::slice(&mat);
        // ... also works as an argument!
        fn receive_sliced(_: HMatRef<f32, HMatRef<i32, ()>>) {}
        receive_sliced(HMatRef::slice(&mat));
        // Access the rows/cols as a reference to the original matrix.
        let f32_row: &Row<f32> = ref_mat.get_row_ref();
        let i32_row: &Row<i32> = ref_mat.get_row_ref();
        assert_eq!(f32_row, &Row::from_iter([None, Some(0.5), None]));
        assert_eq!(i32_row, &Row::from_iter([None, None, Some(-1)]));
        let col: HCol<&f32, HCol<&i32, ()>> = ref_mat.get_col_ref(1);
        assert_eq!(
            col,
            HCol {
                elem: Some(&0.5),
                rem: HCol {
                    elem: None,
                    rem: ()
                }
            }
        )
    }

    #[test]
    fn access_col() {
        let mut mat = h_mat::HMat::new_with::<usize>([Some(0)])
            .extend_with::<f32, _>([Some(0.5)])
            .extend_with::<i32, _>([Some(-5)]);
        {
            // Access a single column as a reference.
            let col_ref: HCol<&i32, HCol<&f32, HCol<&usize, ()>>> = mat.get_col_ref(0);
            assert_eq!(
                col_ref,
                HCol {
                    elem: Some(&-5),
                    rem: HCol {
                        elem: Some(&0.5),
                        rem: HCol {
                            elem: Some(&0),
                            rem: ()
                        }
                    }
                }
            );
        }
        {
            // ... or as a mutable reference...
            let col_mut: HCol<&mut i32, HCol<&mut f32, HCol<&mut usize, ()>>> = mat.get_col_mut(0);
            col_mut.elem.map(|v| *v += 1);
        }
        {
            // ... or directly move it out of the matrix.
            let col: HCol<i32, HCol<f32, HCol<usize, ()>>> = mat.take_col(0);
            assert_eq!(
                col,
                HCol {
                    elem: Some(-4),
                    rem: HCol {
                        elem: Some(0.5),
                        rem: HCol {
                            elem: Some(0),
                            rem: ()
                        }
                    }
                }
            );
            // Then we can place it back to a different position.
            mat.place_col(1, col);
        }
        {
            let col_ref: HCol<&i32, HCol<&f32, HCol<&usize, ()>>> = mat.get_col_ref(0);
            assert_eq!(
                col_ref,
                HCol {
                    elem: None,
                    rem: HCol {
                        elem: None,
                        rem: HCol {
                            elem: None,
                            rem: ()
                        }
                    }
                }
            );
            let col_ref: HCol<&i32, HCol<&f32, HCol<&usize, ()>>> = mat.get_col_ref(1);
            assert_eq!(
                col_ref,
                HCol {
                    elem: Some(&-4),
                    rem: HCol {
                        elem: Some(&0.5),
                        rem: HCol {
                            elem: Some(&0),
                            rem: ()
                        }
                    }
                }
            );
        }
    }

    #[test]
    fn writer() {
        let mut mat = h_mat::HMat::new::<usize>().extend::<f32>().extend::<i32>();
        {
            let ref_mat: HMatRef<f32, HMatRef<i32, ()>> = HMatRef::slice(&mat);
            let mut writer = ref_mat.new_writer();
            // Set the column 0 of the i32 row.
            writer.get_writer().set_col(0, 3);
            // Update the column 0 of the i32 row.
            writer.get_writer().update_col(0, |val: &mut i32| {
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
            GetSubWriter::<i32, _, _>::get_writer(&mut writer).unset_col(0);
            mat.apply(writer);
        }
        {
            let ref_mat: HMatRef<f32, HMatRef<i32, ()>> = HMatRef::slice(&mat);
            assert_eq!(ref_mat.get_row_ref(), &Row::<i32>::from_iter([None]));
        }
    }
}
