# How to Write Tests

- The bodies of test functions usually do three things:
1. Set up any needed data or state
2. Run the code you want to test
3. Assert the results are what you expect

## Test Function Anatomy

- Start each test function as follows:
```rust
#[test]
```

- You can then run your tests with the `cargo test` command.
- Rust will then execute all annotated test functions

- Create a new library with `cargo new adder --lib`.
- You should now get the following in `src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        assert!(true);
    }
}
```

Each test runs on its own thread. When the main thread sees that a test thread has died the test is marked as failed.

- you can run a single test by specifying its name `cargo test it_works`

## Testing Equality with the assert_eq! and assert_ne! Macros

Anything passed into `assert_eq!` or `assert_ne!` must have the `PartialEq` and `Debug` traits implemented. For structs and such it is normally as easy as adding:
```rust
#[derive(PartialEq, Debug)]
```

## Adding Custom Failure Messages

```rust
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
```

## Checking for Panics with `should_panic`

```rust
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        panic!("any panic will do here");
    }
}
```

## Using `Result<T, E>` in Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

- This enables you to use the `?` operator in tests
- In this instance you can no longer use the `#[should_panic]` macro.
