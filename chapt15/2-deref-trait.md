# Treating Smart Pointers Like Regular References with the `Deref` Trait

The `Deref` trait allows you to customise the behaviour of the *dereference operator* `*`.
This allows us to write code that treats smart pointers like regular references.

## Following the Pointer to the Value with the Dereference Operator

```rust
fn main() {
    let x = 5;
    let y = &x; // this is a reference/pointer

    assert_eq!(5, x);
    assert_eq!(5, *y); // here is the dereference operator
}
```

## Using `Box<T>` Like a Reference

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

The difference here to above is that y is assigned to an instance of the `Box` type referencing a copy of the value of x on the heap, rather than y being a reference to the value of x.

## Defining Our Own Smart Pointer

```rust
struct MyBox<T>(T); // this is a tuple struct

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

Now if we try:

```rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

We get the following error:

```bash
$ cargo run
   Compiling deref-example v0.1.0 (file:///projects/deref-example)
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
  --> src/main.rs:14:19
   |
14 |     assert_eq!(5, *y);
   |                   ^^

For more information about this error, try `rustc --explain E0614`.
error: could not compile `deref-example` due to previous error
```

This is because we haven't implemented the `Deref` trait.

## Treating a Type Like a Reference by Implementing the `Deref` Trait

You have to implement the `deref` fn which borrows `self` and returns a reference to the inner data.

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

The `type Target = T;` syntax defines an associated type for the `Deref` trait to use.

Now when `*y` is run the compiler behind the scenes runs this code:
```rust
*(y.deref())
```

Rust substitutes the `*` operator with a call to the `deref` method and then a plain dereference.

## Implicit Deref Coercions with Functions and Methods

*Deref coercion* only works on types that implement the `Deref` trait.

```rust
fn hello(name: &str) {
    println!("Hello {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // This works because of deref coercion
}
```

Without deref coercion we would have to do the following:

```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

The `(*m)` dereferences the `MyBox<String>` into a `String`.
Then the `&` `[..]` takes a string slice of `String` to turn it into `&str`.

Deref coercion means all of this is done automatically for us.

`Deref::deref` will be called as many times as necessary to get the required type for a given parameter.

## How Deref Coercion Interacts with Mutability

You can use the `DerefMut` trait to override the `*` operator on mutable references.

Rust does deref coercion when it finds types and trait implementations in three cases:
- From `&T` to `&U` when `T: Deref<Target=U>`
- From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
- From `&mut T` to `&U` when `T: Deref<Target=U>`
