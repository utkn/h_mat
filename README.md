# h_mat

A type-safe and convenient heterogenous matrix type in Rust. Intended to use for an ECS with compile-time type-checking. 

## Basic usage

### Creation and row access

Use `extend` to build the matrix, and use the `get_row_ref/mut` methods (with type annotations) to gain access to the individual rows.

```rust
// Creating a HMat with i32, f32, usize rows.
let mat: HMat<i32, HMat<f32, HMat<usize, ()>>> = 
    h_mat::HMat::new::<usize>().extend::<f32>().extend::<i32>();
// Access the rows explicitly as a reference.
let usize_row_ref: &Row<usize> = mat.get_row_ref();
let i32_row_ref: &Row<i32> = mat.get_row_ref();
// ... or as a mutable reference.
let mut mat = mat;
let i32_row_mut: &mut Row<i32> = mat.get_row_mut();
```

### Column access

Note that the column types are written explicitly for reference. In general, they are inferred directly from the type of the matrix.

```rust
let mat = h_mat::HMat::new::<usize>().extend::<f32>().extend::<i32>();
// Access a single column as a reference.
let col_ref: HCol<&i32, HCol<&f32, HCol<&usize, ()>>> = mat.get_col_ref(0);
// ... or as a mutable reference...
let mut mat = mat;
let col_mut: HCol<&mut i32, HCol<&mut f32, HCol<&mut usize, ()>>> = mat.get_col_mut(0);
// ... or directly move it out of the matrix.
let col: HCol<i32, HCol<f32, HCol<usize, ()>>> = mat.take_col(0);
// Then we can place it back to a different position.
mat.place_col(1, col);
```

### Reforming

We can invoke `HMatRef::reform` to extract a reference matrix with arbitrary row order, i.e., a `HMatRef`, whose fields are indicated by the type annotation either at the let binding or the parameter. 

```rust
let mat = h_mat::HMat::new::<usize>().extend::<f32>().extend::<i32>();
// Reform as a heterogenous matrix of f32, and i32 rows.
let mat_ref: HMatRef<f32, HMatRef<i32, ()>> = HMatRef::reform(&mat);
// ... also works as an argument!
fn receive_reformed(_: HMatRef<f32, HMatRef<i32, ()>>) {}
receive_reformed(HMatRef::reform(&mat));
// Of course, we can access the rows/cols of the original matrix.
let i32_row_ref: &Row<i32> = mat_ref.get_row_ref();
let first_col_ref = mat_ref.get_col_ref(0);
```
