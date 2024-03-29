# mathematica

A bunch of useful mathematical functions and operations, in rust!!

for now there's only vectors & matrices

step by step for newbies:

```bash
cargo new <anyname> --bin
```

now open <anyname/src>
in Cargo.toml add:

```toml
[dependecies]
mathematica = { git = "https://github.com/BilakshanP/mathematica" }
```

now u may use the following code in main.rs:

```rust
use mathematica::vectors::Vector as v;
use mathematica::matrices::Matrix as m;

fn main() {
    let vector_1 = v::new(1.0, 2.0, 3.0);
    let vector_2 = v::new(5.0, 6.0, 9.0);

    println!("Vector 1: {}\nVector 2: {}", vector_1, vector_2);
    println!("Vector/Cross product of Vector 1 & 2: {}", vector_1.cross(&vector_2));

    let matrix_1 = m::new_from_vec_sized(
        vec![
            1.0, 2.0, 3.0,
            4.0, 7.0, 8.0,
            2.0, 6.0, 4.0
        ], 3, 3).unwrap();
    let matrix_2 = m::new_identity_matrix(3).times(2.0);
    let matrix_3 = matrix_1.transpose().cross(&matrix_2).unwrap();

    println!("Matrix 1:\n{}\n", matrix_1);
    println!("Matrix 2:\n{}\n", matrix_2);

    println!(
        "Matrix/Cross product of Matrix 1's transpose and an Identity matrix of same size times 2:\n{}\n",
        matrix_3
    );

    println!("Determinant of above matrix is: {}", matrix_3.determinant());
}
```
