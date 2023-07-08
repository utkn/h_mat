use crate::{HMat, HMatRef};

use super::Row;

/// Internal type used for the recursive implementations of the `AccessRowRef` and `AccessRowMut` traits.
#[derive(Clone, Copy, Debug)]
pub struct AccessRowDirective<T>(T);

/// Represents a type whose rows can be accessed as a reference.
pub trait AccessRowRef<D, Directive> {
    /// Returns a reference to the `Row<D>`.
    fn get_row_ref(&self) -> &Row<D>;
}

/// Represents a type whose rows can be accessed as a mutable reference.
pub trait AccessRowMut<D, Directive> {
    /// Returns a mutable reference to the `Row<D>`.
    fn get_row_mut(&mut self) -> &mut Row<D>;
}

impl<'a, D, R> AccessRowRef<D, ()> for HMatRef<'a, D, R> {
    fn get_row_ref(&self) -> &Row<D> {
        &self.row
    }
}

impl<'a, T, R, D, A> AccessRowRef<D, AccessRowDirective<A>> for HMatRef<'a, T, R>
where
    R: AccessRowRef<D, A>,
{
    fn get_row_ref(&self) -> &Row<D> {
        self.rem.get_row_ref()
    }
}

// Implementation of AccessRowRef for HMat
impl<D, R> AccessRowRef<D, ()> for HMat<D, R> {
    fn get_row_ref(&self) -> &Row<D> {
        &self.head_row
    }
}

impl<T, R, D, A> AccessRowRef<D, AccessRowDirective<A>> for HMat<T, R>
where
    R: AccessRowRef<D, A>,
{
    fn get_row_ref(&self) -> &Row<D> {
        self.rem.get_row_ref()
    }
}

// Implementation of AccessRowMut for HMat
impl<D, R> AccessRowMut<D, ()> for HMat<D, R> {
    fn get_row_mut(&mut self) -> &mut Row<D> {
        &mut self.head_row
    }
}

impl<T, R, D, A> AccessRowMut<D, AccessRowDirective<A>> for HMat<T, R>
where
    R: AccessRowMut<D, A>,
{
    fn get_row_mut(&mut self) -> &mut Row<D> {
        self.rem.get_row_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn access_row() {
        let mut mat = HMat::<usize, ()>::new().extend::<f32>().extend::<i32>();
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
}
