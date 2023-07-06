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
    fn reform_inference() {
        let hmat = h_mat::HMat::<usize, _>::new()
            .extend::<f32>()
            .extend::<i32>();
        // Call with && for auto type inference.
        let reformed: HMatRef<f32, HMatRef<i32, ()>> = (&&hmat).reform();
        fn receive_reformed(r: HMatRef<f32, HMatRef<i32, ()>>) {
            r;
        }
        // ... also works as an argument!
        receive_reformed((&&hmat).reform())
    }
}
