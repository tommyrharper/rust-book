# Using `Box<T>` to Point to Data on the Heap

This allows you to store data on the heap rather than the stack.
The pointer remains on the stack.
Other than this they don't have any performance overhead.

Common use cases:
- When you have a type whose size cannot be known at compile time.
- When you have a large amount of data and you want to transfer ownership but ensure the data won't be copied when you do so.
- When you want to own a value and you only care that it implements a specific type.

## Using a `Box<T>` to Store Data on the Heap

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

## Enabling Recursive Types with Boxes

```rust
// This won't compile
enum List {
    Cons(i32, List),
    Nil,
}
```

```rust
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

This throws:

```bash
$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
error[E0072]: recursive type `List` has infinite size
 --> src/main.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^ recursive type has infinite size
2 |     Cons(i32, List),
  |               ---- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +

error[E0391]: cycle detected when computing drop-check constraints for `List`
 --> src/main.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^
  |
  = note: ...which immediately requires computing drop-check constraints for `List` again
  = note: cycle used when computing dropck types for `Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing }, value: List } }`

Some errors have detailed explanations: E0072, E0391.
For more information about an error, try `rustc --explain E0072`.
error: could not compile `cons-list` due to 2 previous errors
```

The solution is to use a `Box`:

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

The `Box<T>` type is a smart pointer because it implements the `Deref` trait.
This means `Box<T>` values can be treated like references.
