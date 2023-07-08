use std::marker::PhantomData;

use crate::{AccessRowRef, HMatRef};

/// Internal type used for the recursive implementations of the `Slicer` trait.
#[derive(Clone, Copy, Debug)]
pub struct SlicerDirective<Head, Tail>(PhantomData<*const Head>, PhantomData<*const Tail>);

/// Represents a type that can construct itself by rearranging the fields of the original type `H`.
pub trait Slicer<'a, H, D, Directive> {
    fn slice(h: &'a H) -> Self;
}

impl<'a, H, D, A> Slicer<'a, H, D, SlicerDirective<A, ()>> for HMatRef<'a, D, ()>
where
    H: AccessRowRef<D, A>,
{
    fn slice(h: &'a H) -> Self {
        HMatRef {
            row: h.get_row_ref(),
            rem: (),
        }
    }
}

impl<'a, H, D1, D2, R, A1, A2, Tail>
    Slicer<'a, H, D1, SlicerDirective<A1, SlicerDirective<A2, Tail>>>
    for HMatRef<'a, D1, HMatRef<'a, D2, R>>
where
    H: AccessRowRef<D1, A1>,
    H: AccessRowRef<D2, A2>,
    HMatRef<'a, D2, R>: Slicer<'a, H, D2, SlicerDirective<A2, Tail>>,
{
    fn slice(h: &'a H) -> Self {
        HMatRef {
            row: h.get_row_ref(),
            rem: <HMatRef<'a, D2, R> as Slicer<'a, H, D2, SlicerDirective<A2, Tail>>>::slice(h),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn slice() {
        let mat = HMat::<usize, ()>::new_with([Some(0), Some(1), Some(2)])
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
}
