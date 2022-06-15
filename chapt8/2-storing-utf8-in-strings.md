# Storing utf8 in strings

- Many of the `Vec` operations are available with `String` as well:
```rust
let mut s = String::new();
```

- We can also use the `to_string` method that exists on any type that implements the `Display` trait.

```rust
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();

let s = String::from("initial contents");
```

## Appending to a string

- This is all valid:

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);

let mut s = String::from("lo");
s.push('l');
```

## Concatenating with the `+` Operator and `format!` Macro

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

The + add signature looks something like this:
```rust
fn add(self, s: &str) -> String {..}
```

- Here we are adding a reference of the second string to the first string.
- So how come we can pass `&String` type into a `&str` param?
  - Rust uses a *deref coercion* which turns `&s2` into `&s2[..]` - it slices the whole string
- `s2` remains valid after the operation as no ownership is taken.
- `s1` is no longer valid - it is not copied, this is more efficient.

This:
```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```
Can be expressed like this:
```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3); // returns String
```
- `format!` does not take ownership of any of it's arguments.

## Indexing Strings

Cannot do this:
```rust
let s1 = String::from("hello");
let h = s1[0];
```

Will throw:
```bash
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0277]: the type `String` cannot be indexed by `{integer}`
 --> src/main.rs:3:13
  |
3 |     let h = s1[0];
  |             ^^^^^ `String` cannot be indexed by `{integer}`
  |
  = help: the trait `Index<{integer}>` is not implemented for `String`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `collections` due to previous error
```

## Internal Representation

- A `String` is a wrapper of a `Vec<u8>`.
```rust
let hello = String::from("Hola"); // len is 4
let hello = String::from("Здравствуйте"); // len is 24
```

## Bytes and Scalar Values and Grapheme Clusters

If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values that looks like this:
```rust
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```
Look at it as unicode scalar values (Rust's `char` type):
```rust
['न', 'म', 'स', '्', 'त', 'े']
```
Char 4 and 6 are diacritics or accents, so what a person would see, or a `grapheme cluster` is as follows:
```rust
["न", "म", "स्", "ते"]
```

## Slicing strings

- you can slice strings using ranges - just be careful, some characters have more than 1 byte, and slicing between multi-byte characters will cause the thread to panic.

```rust
let hello = "Здравствуйте";

let s = &hello[0..4]; // Зд
```

## Iterating over strings

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

Prints out:
```
न
म
स
्
त
े
```

```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

prints out:
```
224
164
// --snip--
165
135
```

