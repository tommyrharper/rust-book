# Bringing Paths into Scope with the `use` Keyword

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```

- It is idiomatic to leave in the parent scope of a function when using the `use` keyword:
  - e.g. `hosting::add_to_waitlist();` instead of `add_to_waitlist();`
  - This makes it clear the function isn't locally defined
- For structs, enums and other items with `use`, it is idiomatic to specify the full path, like so:
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```
- Unless there are two items with the same name in scope.

## Providing new names with the `as` keyword

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

## Re-exporting Names with pub `use`

- This is known as re-exporting
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
- Before users of the api would have to: `restaurant::front_of_house::hosting::add_to_waitlist()`
  - But now they can: `restaurant::hosting::add_to_waitlist()`

## Using External Packages

- Add to `Cargo.toml`: `rand = "0.8.3"`
Then:
```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
}
```

- View more crates at [crates.io](crates.io)
- The standard library (`std`) is also a crate that's external to our package, shipped with the Rust language:
```rust
use std::collections:HashMap;
```

## Using Nested Paths to Clean Up Large `use` Lists

- This takes up a lot of space:
```rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
```
- You can condense it as follows:
```rust
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

- Or:
```rust
use std::io;
use std::io::Write;
```
- To:
```rust
use std::io::{self, Write};
```

## The Glob Operator

- Bring all things in from a certain path:
```rust
use std::collections::*;
```
- Be careful with this pattern as it makes it harder to tell what names are in scope.
