use crate::{HMat, Row};

/// Represents a type that can be extended with new types.
pub trait Extend {
    type Old;
    /// Extends this `HMat<T, _>` with a new empty row of type `E`, returning `HMat<E, HMat<T, _>>`.
    fn extend<E>(self) -> HMat<E, Self::Old>;
    /// Extends this `HMat<T, _>` with a new row of type `E`, initialized with the given elements, returning `HMat<E, HMat<T, _>>`.
    fn extend_with<E, I>(self, iter: I) -> HMat<E, Self::Old>
    where
        I: IntoIterator<Item = Option<E>>;
}

impl<T, R> Extend for HMat<T, R> {
    type Old = HMat<T, R>;
    fn extend<E>(self) -> HMat<E, Self::Old> {
        HMat {
            head_row: Default::default(),
            rem: self,
        }
    }

    fn extend_with<E, I>(self, iter: I) -> HMat<E, Self::Old>
    where
        I: IntoIterator<Item = Option<E>>,
    {
        HMat {
            head_row: Row::from_iter(iter),
            rem: self,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn extend() {
        // Creating a HMat with i32, f32, usize rows.
        let mat = HMat::<usize, ()>::new().extend::<f32>().extend::<i32>();
        assert_eq!(mat.head_row, Row::<i32>::default());
        assert_eq!(mat.rem.head_row, Row::<f32>::default());
        assert_eq!(mat.rem.rem.head_row, Row::<usize>::default());
        assert_eq!(mat.rem.rem.rem, ());
    }
}
