# Chapt 6 - Enums and Pattern matching

- Enums = *enumerations*
  - Define a type by enumerating its possible *variants*
  - Similar to *algebraic data types* in F#, OCaml and Haskell

```rust
// Declare an enum
enum IpAddrKind {
    V4,
    V6,
}

// Create instances of an enum
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

// Set an enum as a parameter type
fn route(ip_kind: IpAddrKind) {}

route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

You can store data in enums:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

You can put any data type inside an enum, e.g. a struct:

```rust

struct Ipv4Addr {
    // ....
}

struct Ipv6Addr {
    // ....
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```


You can do pretty much anything in an enum:

```rust
// enum version
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// struct version
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

- The problem with using structs in the above example is it would be difficult to define a function that could accept all of those values.
- Using a Message enum clearly defines them all within a single type that can be passed around.

- You can also have methods on enums:

```rust
impl Message {
    fn call(&self) {
        // ...
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## The option enum

- Deals with the situation in which a value could be something or nothing.
- You don't need to import it, it's always available. (it's in the prelude)
  - It's variants `None` and `Some` are also in the prelude
    - Hence you don't have to do this `Option::None` or `Option::Some(3)`
      - You can just do this: `None` or `Some(3)`
- There is no `null` in rust.

```rust
enum Option<T> {
    None,
    Some(T),
}
```

```rust
let some_number = Some(5);
let some_string = Some("yo");

// Must specify the type as it cannot be inferred
let absent_number: Option<i32> = None;
```






