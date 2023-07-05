use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Row<T>(pub(super) Vec<Option<T>>);

impl<T> Default for Row<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Row<T> {
    pub fn get(&self, idx: usize) -> Option<&T> {
        self.0.get(idx).map(|opt_elem| opt_elem.as_ref()).flatten()
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.0
            .get_mut(idx)
            .map(|opt_elem| opt_elem.as_mut())
            .flatten()
    }

    pub fn take(&mut self, idx: usize) -> Option<T> {
        self.0
            .get_mut(idx)
            .map(|opt_elem| opt_elem.take())
            .flatten()
    }

    pub fn place(&mut self, idx: usize, new_elem: T) -> Option<T> {
        self.0.resize_with(idx + 1, || None);
        self.0.get_mut(idx).unwrap().replace(new_elem)
    }
}
