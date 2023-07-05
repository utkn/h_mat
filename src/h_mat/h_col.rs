use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HCol<T, R> {
    pub(super) elem: Option<T>,
    pub(super) rem: R,
}

impl<T, R> HCol<T, R> {
    pub fn get(&self) -> Option<&T> {
        self.elem.as_ref()
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.elem.as_mut()
    }

    pub fn take(&mut self) -> Option<T> {
        self.elem.take()
    }

    pub fn place(&mut self, new_elem: T) -> Option<T> {
        self.elem.replace(new_elem)
    }
}
