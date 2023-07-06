mod h_mat;

pub use crate::h_mat::*;

#[cfg(test)]
mod tests {
    use crate::{Extend, HMatRef, Reform};

    use super::*;

    #[test]
    fn extend() {
        // Creating a HMat with i32, f32, usize rows.
        let _ = h_mat::HMat::new::<usize>().extend::<f32>().extend::<i32>();
    }

    #[test]
    fn reform_inference() {
        let mat = h_mat::HMat::new::<usize>().extend::<f32>().extend::<i32>();
        // Call with && for auto type inference in let bindings.
        let _: HMatRef<f32, HMatRef<i32, ()>> = (&&mat).reform();
        // ... also works as an argument!
        fn receive_reformed(_: HMatRef<f32, HMatRef<i32, ()>>) {}
        receive_reformed((&&mat).reform())
    }
}
