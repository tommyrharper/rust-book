# Chapt 5 - Structs

A struct is like an object's data attributes.
You can have a method on a struct.

## Defining and Instantiating Structs

Structs are similar to tuples in that both hold multiple related values.

The pieces of a struct can be different types.

In a struct you will name each piece of data using a key.

Define a struct as follows:
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

To use a struct once defined, create an *instance* of it:
```rust
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

- An entire instance must be mutable, you cannot mark certain fields as mutable and others as not.

Here is an example of a `build_user` function.
```rust
fn build_user(email: String, username: String) -> User {
    User {
        email, // you could also use email: email, but this is ugly
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

## Struct Update Syntax
You can do this:
```rust
fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```
Like this:
```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

- But note, `user1` is now unusable because the username field was *moved* into `user2`.
- You would have to clone the `username` field in order to be able to use both.

## Tuple Structs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0); // origin is a distinct type from black
}
```

## Unit Structs

- Behave similar to the unit type `()`.

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

## Ownership of Struct Data

- If you store a reference to some data owned by something else in a struct, rust ensures that the data referenced by a struct is valid for as long as the struct is, using `lifetimes` - discussed further in chapt 10.

Hence this won't work:
```rust
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```
Rust will complain:
```bash
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
error[E0106]: missing lifetime specifier
 --> src/main.rs:3:15
  |
3 |     username: &str,
  |               ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 ~     username: &'a str,
  |

error[E0106]: missing lifetime specifier
 --> src/main.rs:4:12
  |
4 |     email: &str,
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 |     username: &str,
4 ~     email: &'a str,
  |

For more information about this error, try `rustc --explain E0106`.
error: could not compile `structs` due to 2 previous errors
```

Later on you will learn how to fix this error using lifetimes, but for now we will just give the struct ownership of all of it's data using owned types like `String` instead of refernces like `&str`.

## Printing structs

- For debugging purposes you can do the following:
```rust
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age: i32
}

fn main() {
    let user1 = User {
        username: String::from("tom"),
        email: String::from("tom@tom.com"),
        age: 25,
    };

    println!("user1: {:?}", user1);
    println!("user1: {:#?}", user1); // {:#?} pretty prints

    dbg!(&user1);
}
```
Note the `#[derive(Debug)]`, this allows you to print the struct.

You can also use `dbg!`. This takes ownership of the expression, prints it with file and line number, and returns ownership.
