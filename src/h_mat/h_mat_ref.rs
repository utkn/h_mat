use super::Row;

#[derive(Clone, Copy, Debug)]
pub struct HMatRef<'a, D, R> {
    pub row: &'a Row<D>,
    pub rem: R,
}
