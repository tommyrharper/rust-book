# Validating References with Lifetimes

- Lifetimes ensure that references are valid as long as we need them to be.
- Every reference in rust has a *lifetime*
  - This is the scope for which that reference is valid
  - This is usually implicit and inferred
  - If there are multiple possible lifetimes, we must annotate them

## Preventing Dangling References with Lifetimes

- *Dangling references* cause a program to reference data other than the data it is intended to reference.
- Rust uses the borrow checker to ensure that no values are used outside of their lifetime.

## Generic Lifetimes in Functions

This function:
```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Returns the following error:
```bash
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `chapter10` due to previous error
```

- This is because the function does not know which reference will be returned and hence it does not know the lifetime of the returned value.

## Lifetime annotation syntax

- This doesn't change how long the references live.
- Describes the relationship of lifetimes between different references
- `'a` is a common annotation used.
  - Must start with an `'`
  - Usually short notation like generic types

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

## Lifetime annotation in Function Signatures

- This says the returned reference will be valid as long as both the input references are valid:
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
- This means the returned value will have the shorter of the two lifetimes, and hence this code will not compile:

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

## Thinking in terms of lifetimes

- You can also do this, if the same reference is always returned:
```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

## Lifetime Annotations in Struct Definitions

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

## Lifetime Elision

- You don't have to use lifetime annotation in the following situation:

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

- This is just because it has programmed to happen automatically in the rust compiler
- These are called *lifetime elision rules*

- Lifetimes on params are called *input lifetimes*
- Lifetimes on return values are called *output lifetimes*

The rules:
1. The compiler assigns a lifetime parameter to each parameter that is a reference.
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because it is a method, the lifetime of `self` is assigned to all output lifetime parameters.

## Lifetime annotations in method definitions

- The lifetime parameter doesn't need to be added to the method definition due to rule 3.
```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

## The Static Lifetime

- There is a special case - `'static`
  - This denotes the reference can live for the entire duration of the program.
  - All string literals have the `'static` lifetime, denoted as follows:

```rust
let s: &'static str = "I have a static lifetime.";
```
- Be careful around error messages suggesting you use a `'static` lifetime, often that is not really the best solution and is due to a dangling reference or mismatch of available lifetimes. Try to fix the problem at its root.

## Generic Type Parameters, Trait Bounds, and Lifetimes Together

- Mixing all we have learned together:
```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
