# Storing Lists of Values with Vectors

- Can only store one type

```rust
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];
```

## Updating a Vector

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

## Dropping a Vector drops its elements

```rust
{
    let v = vec![1, 2, 3, 4];

    // do stuff with v
} // <- v goes out of scope and is freed here
```

## Reading elements of Vectors

- Two ways:
  - Indexing
    - Uses a reference
    - Crashes/panics if el does not exist
  - Using the get method
    - Returns `None` if element does not exist

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

- The rules of not being able to have mutable and immutable references in the same scope remain, hence the following code is not allowed:

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {}", first);
```

It will throw the following error:

```bash
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 | 
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 | 
8 |     println!("The first element is: {}", first);
  |                                          ----- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `collections` due to previous error
```

## Iterating over a Vector

- immutable:
```rust
let v  = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```
- mutable:
```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50; // notice * the dereference operator
}
```

## Using an enum to store multiple types

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

- If you don't have an exhaustive list of types you can use a trait object - covered in chapter 17.
