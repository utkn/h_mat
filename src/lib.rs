mod h_mat;

#[cfg(test)]
mod tests {
    use crate::h_mat::{Extend, HMatRef, Reformable};

    use super::*;

    #[test]
    fn extend() {
        let hmat = h_mat::HMat::<usize, _>::new()
            .extend::<f32>()
            .extend::<i32>();
    }

    #[test]
    fn reform() {
        let hmat = h_mat::HMat::<usize, _>::new()
            .extend::<f32>()
            .extend::<i32>();
        let reformed: HMatRef<f32, HMatRef<i32, ()>> = hmat.reform();
    }
}
