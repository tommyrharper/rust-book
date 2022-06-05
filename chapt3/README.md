# Chapt 3

## Constants

- Can be in global scope
- Cannot be prefixed with `mut`
- Type must be declared
- Can only be set to a constant expression, no the result ofa function call or anything that could be determined at runtime

## Shadowing

- Updates the value assigned to a variable within the block scope of the "shadow"
- Reassigning as opposed to shadowing, will change the value of a variable within the variables original scope

## Scalar Types

- integers
- floating-point numbers
- booleans
- characters

### Integer Types

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

- signed
  - means it must have a sign to determine whether it is negative or positive
  - stored using [Two's Complement](https://en.wikipedia.org/wiki/Two%27s_complement) representation
- unsigned
  - can only be positive, it doesn't have a +/- sign
- isize and usize
  - depends on computer architecture (e.g. arch) (64 bits or 32 bits depending on machine)

#### Integer Literals

| Number literals | Example     |
| --------------- | ----------- |
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |

- rust defaults to `i32`.

## Chars

- single quote

```rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
```

## Tuples

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
y.0 // 500
```

- Fixed length once declared
- Can mix types within a tuple
- Can be destructured

## Array

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
let b = [3; 5]; // creates [3, 3, 3, 3, 3] - array of 3s of length 5
let first = a[0];
```

- Fixed length
- All elements must have the same type
- Allocates data on the stack rather than the heap

## Functions

- statements don't return values
  - e.g. `let x = 6;`
- expressions do return values
  - e.g. `5 + 6`
  - a newly scoped block created curly brackets is an expression `{ ... }`
```rust
let y = {
    let x = 3;
    x + 1 // if you add a semicolon, this ceases to be an expression and returns nothing ()
};

println!("The value of y is: {}", y); // prints 4
```

## Loops

- loop
- while
- for
