use serde::{Deserialize, Serialize};

/// A homogenous row, in which all the elements of the same type. Elements are stored as `Option<T>` types, as they can be `None` or some value `Some(T)`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Row<T>(pub(super) Vec<Option<T>>);

impl<T> Default for Row<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Row<T> {
    /// Returns the element at the given index.
    pub fn get(&self, idx: usize) -> Option<&T> {
        self.0.get(idx).map(|opt_elem| opt_elem.as_ref()).flatten()
    }

    /// Returns the element at the given index as a mutable reference.
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.0
            .get_mut(idx)
            .map(|opt_elem| opt_elem.as_mut())
            .flatten()
    }

    /// Removes the element at the given index and returns it. If an element was indeed removed, the corresponding value at the underlying row will be set to `None`.
    pub fn take(&mut self, idx: usize) -> Option<T> {
        self.0
            .get_mut(idx)
            .map(|opt_elem| opt_elem.take())
            .flatten()
    }

    /// Places a value to the given index. Returns the old value that was at the given index.
    pub fn place(&mut self, idx: usize, new_elem: T) -> Option<T> {
        self.0.resize_with(idx + 1, || None);
        self.0.get_mut(idx).unwrap().replace(new_elem)
    }
}
