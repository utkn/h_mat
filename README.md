# h_mat

A type-safe and convenient heterogenous matrix type in Rust. Intended to be used for an ECS with compile-time type-checking. 

A `HMat`, in this context, means *a list of vectors of arbitrary types*, e.g., a `HMat` with three rows can be `(Vec<Option<T1>>, Vec<Option<T2>>, Vec<Option<T3>>)` where `T1 != T2 != T3`. In an ECS setting, `HMat` would represent the game state, a row would store the instances of a component in the contiguous memory, whereas a column would correspond to an entity.

## Basic usage

### Creation and row access

Use `extend` to build the matrix, and use the `get_row_ref/mut` methods (with type annotations) to gain access to the individual rows.

```rust
// Creating a HMat with i32, f32, usize rows.
let mat: HMat<i32, HMat<f32, HMat<usize, ()>>> = 
    HMat::new::<usize>().extend::<f32>().extend::<i32>();
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
let mat = HMat::new::<usize>().extend::<f32>().extend::<i32>();
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
let mat = HMat::new::<usize>().extend::<f32>().extend::<i32>();
// Reform as a heterogenous matrix of f32, and i32 rows.
let mat_ref: HMatRef<f32, HMatRef<i32, ()>> = HMatRef::reform(&mat);
// ... also works as an argument!
fn receive_reformed(_: HMatRef<f32, HMatRef<i32, ()>>) {}
receive_reformed(HMatRef::reform(&mat));
// Of course, we can access the rows/cols of the original matrix.
let i32_row_ref: &Row<i32> = mat_ref.get_row_ref();
let first_col_ref = mat_ref.get_col_ref(0);
```

### Writing

Other than calling the methods that return mutable references to the underlying objects, it is possible to collect the modifications to be applied in the future. This is useful, since it is not possible to mutate the original matrix while holding a `HMatRef` pointing to that matrix.

```rust
let mut mat = h_mat::HMat::new::<usize>().extend::<f32>().extend::<i32>();
let ref_mat: HMatRef<f32, HMatRef<i32, ()>> = HMatRef::reform(&mat);
// Create a new writer from the `HMatRef`.
let mut writer = ref_mat.new_writer();
// Set the column 0 of the i32 row.
writer.get_writer().set_col(0, 3);
// Update the column 0 of the i32 row.
writer.get_writer().update_col(0, |val: &mut i32| {
    *val += 1;
});
// Apply the modifications at once. This is the only place where we borrow `mat` by mutable reference.
mat.apply(writer);
```
