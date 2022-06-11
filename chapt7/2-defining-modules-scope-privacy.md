# Defining Modules to control scope and privacy

- `use` to bring things into scope
- `pub` to make them public


## Two types of paths

- absolute paths
  - Starts from crate root, for own crate this is literally `crate`, for other crates it is the crate name.
- relative paths
  - Starts from the current module and uses `self`, `super` or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

This throws the following error:

```bash
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^ private module
  |
note: the module `hosting` is defined here
 --> src/lib.rs:2:5
  |
2 |     mod hosting {
  |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^ private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors
```

- This is because modules define Rust's *privacy boundary* - the line that encapsulates the implementation details external code isn't allowed to know about, call or rely on.
- If you want to make an item e.g. function/struct private, put it in a module.
- In rust all items are private by default.
  - Items in a parent module can't use private items inside child modules.
  - Items in child modules can use the items in their ancestor modules.
    - This is so child modules can wrap and hide their implementation details.
    - While they also know the context within which they are defined.
      - Metaphor: "back-end of restaurant knows whats going on in the front-end, but customers cannot see the back-end"

- If you want to expose inner child modules to outer modules, you must use the pub keyword.

## The `pub` keyword

To make the above code work, we must publish the private module and function:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

## Best practice for packages with a binary and a library

- Write the code as if you are both writing the client and the api.
- The api goes in `lib.rs` and the client goes in `main.rs`
- Typically you have just enough code in the binary crate to start an executable that calls code within the library crate.
- This lets other users benefit the most from your package as they have access to all the functionality within `lib.rs`

## Starting relative paths with `super`

- this is like starting a filesystem path with `..` syntax.

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

## Making Structs and Enums Public

- Structs can have both private and public fields
- If you have a private field, you need some appropriately scoped factory function to build the struct

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        // seasonal fruit is private
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

- Conversely, a public `enum` has all of its variants as public.

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

