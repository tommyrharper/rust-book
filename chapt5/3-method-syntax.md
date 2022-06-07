# Method syntax

- Methods are defined within the context of a struct, enum or trait object.
- Their first parameter is always `self`.
  - Self can be moved and borrowed immutably or mutably.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // note &self here, we only want to borrow self
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

- methods can have the same name as properties:

```rust
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```
This is often used for getters, where the method returns the value of the property. This allows you to make the field private but the method public and thus enable read-only access to that field as part of the types public API.

Public and private will be further discussed in chapt 7.

In terms of references the following are the same:
```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

## Associated functions

- All functions defined with an `impl` block are called `associated functions` as they are associated with the type being implemented.
- We can have an associated function that is not a method as its first parameter is not `self`, e.g. `String::from`.

```rust
impl Rectangle {
    // associated function but not a method as no self is passed in
    // called using Rectangle::square(3)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

## Multiple impl Blocks

This is allowed:
```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

You will see a reason to do this in chapt 10.
