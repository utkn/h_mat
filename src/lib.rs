mod h_mat;

#[cfg(test)]
mod tests {
    use crate::h_mat::Extend;

    use super::*;

    #[test]
    fn extend() {
        let hmat = h_mat::HMat::<usize, _>::new()
            .extend::<f32>()
            .extend::<i32>();
    }
}
