use super::Row;

/// Internal type used for the recursive implementations of `AccessRowRef` and `AccessRowMut` traits.
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
