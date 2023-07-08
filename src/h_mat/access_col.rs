use crate::{HMat, HMatRef};

use super::HCol;

/// Represents a type whose columns can be accessed as a reference.
pub trait AccessColRef<'a, T> {
    type Rem;
    /// Returns the column with the given index `idx` as a reference.
    fn get_col_ref(&'a self, idx: usize) -> HCol<&T, Self::Rem>;
}

/// Represents a type whose columns can be accessed as a mutable reference.
pub trait AccessColMut<'a, T> {
    type Rem;
    /// Returns the column with the given index `idx` as a mutable reference.
    fn get_col_mut(&'a mut self, idx: usize) -> HCol<&mut T, Self::Rem>;
}

/// Represents a type whose columns can be moved out.
pub trait TakeCol<T> {
    type Rem;
    /// Removes the column with the given index `idx` and returns it. The corresponding values in the original matrix will be set to `None` after this operation.
    fn take_col(&mut self, idx: usize) -> HCol<T, Self::Rem>;
}
/// Represents a type that can store a column.
pub trait PlaceCol<T> {
    type Rem;
    /// Places the given column `col` at the given index `idx`.
    fn place_col(&mut self, idx: usize, col: HCol<T, Self::Rem>);
}

impl<'a, T1, T2, R> AccessColRef<'a, T1> for HMatRef<'a, T1, HMatRef<'a, T2, R>>
where
    HMatRef<'a, T2, R>: AccessColRef<'a, T2>,
{
    type Rem = HCol<&'a T2, <HMatRef<'a, T2, R> as AccessColRef<'a, T2>>::Rem>;
    fn get_col_ref(&'a self, idx: usize) -> HCol<&T1, Self::Rem> {
        HCol {
            elem: self.row.get(idx),
            rem: self.rem.get_col_ref(idx),
        }
    }
}

impl<'a, T> AccessColRef<'a, T> for HMatRef<'a, T, ()> {
    type Rem = ();
    fn get_col_ref(&'a self, idx: usize) -> HCol<&T, Self::Rem> {
        HCol {
            elem: self.row.get(idx),
            rem: (),
        }
    }
}

impl<'a, T1, T2, R> AccessColRef<'a, T1> for HMat<T1, HMat<T2, R>>
where
    Self: 'a,
    HMat<T2, R>: AccessColRef<'a, T2>,
{
    type Rem = HCol<&'a T2, <HMat<T2, R> as AccessColRef<'a, T2>>::Rem>;
    fn get_col_ref(&'a self, idx: usize) -> HCol<&T1, Self::Rem> {
        HCol {
            elem: self.head_row.get(idx),
            rem: self.rem.get_col_ref(idx),
        }
    }
}

impl<'a, T> AccessColRef<'a, T> for HMat<T, ()> {
    type Rem = ();
    fn get_col_ref(&'a self, idx: usize) -> HCol<&T, Self::Rem> {
        HCol {
            elem: self.head_row.get(idx),
            rem: (),
        }
    }
}

impl<'a, T1, T2, R> AccessColMut<'a, T1> for HMat<T1, HMat<T2, R>>
where
    Self: 'a,
    HMat<T2, R>: AccessColMut<'a, T2>,
{
    type Rem = HCol<&'a mut T2, <HMat<T2, R> as AccessColMut<'a, T2>>::Rem>;
    fn get_col_mut(&'a mut self, idx: usize) -> HCol<&mut T1, Self::Rem> {
        HCol {
            elem: self.head_row.get_mut(idx),
            rem: self.rem.get_col_mut(idx),
        }
    }
}

impl<'a, T> AccessColMut<'a, T> for HMat<T, ()> {
    type Rem = ();
    fn get_col_mut(&mut self, idx: usize) -> HCol<&mut T, Self::Rem> {
        HCol {
            elem: self.head_row.get_mut(idx),
            rem: (),
        }
    }
}

impl<T1, T2, R> TakeCol<T1> for HMat<T1, HMat<T2, R>>
where
    HMat<T2, R>: TakeCol<T2>,
{
    type Rem = HCol<T2, <HMat<T2, R> as TakeCol<T2>>::Rem>;
    fn take_col(&mut self, idx: usize) -> HCol<T1, Self::Rem> {
        HCol {
            elem: self.head_row.take(idx),
            rem: self.rem.take_col(idx),
        }
    }
}
impl<T> TakeCol<T> for HMat<T, ()> {
    type Rem = ();
    fn take_col(&mut self, idx: usize) -> HCol<T, Self::Rem> {
        HCol {
            elem: self.head_row.take(idx),
            rem: (),
        }
    }
}

impl<T1, T2, R> PlaceCol<T1> for HMat<T1, HMat<T2, R>>
where
    HMat<T2, R>: PlaceCol<T2>,
{
    type Rem = HCol<T2, <HMat<T2, R> as PlaceCol<T2>>::Rem>;

    fn place_col(&mut self, idx: usize, col: HCol<T1, Self::Rem>) {
        if let Some(elem) = col.elem {
            self.head_row.place(idx, elem);
        }
        self.rem.place_col(idx, col.rem);
    }
}

impl<T> PlaceCol<T> for HMat<T, ()> {
    type Rem = ();

    fn place_col(&mut self, idx: usize, col: HCol<T, Self::Rem>) {
        if let Some(elem) = col.elem {
            self.head_row.place(idx, elem);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn access_col_h_mat() {
        let mut mat = HMat::<usize, ()>::new_with([Some(0)])
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
}
