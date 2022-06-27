# Test Organisation

Two types of tests in rust:
1. Unit tests
2. Integration tests

## Unit Tests

- Test code in isolation from the rest of the code.
- Place unit tests in the `src` directory in each file with the code that they're testing.
- The convention is to create a module named `tests` in each file.
  - Annotate that module with `#[cfg(test)]`
    - This tells rust to only compile that code with `cargo test`, not `cargo build`.
    - `cfg` stands for *configuration*
      - This tells rust that the following item should only be included for a certain configuration option.
        - In this case that configuration option is `test`.

## Testing Private Functions

- You can test private modules by going into their child scope as follows:

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    user super::*;

    #[test]
    fn internal() {
        // this is allowed because the internal_adder function comes from a parent scope
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

## Integration Tests

- Integration tests are external to your library. They use your library in the same way any other code would, which means they can only call functions that are part of your library's public API.
- To create integration tests:
  - Create `tests` directory.

Filename: `root_library_folder/tests/integration_test.rs`
```rust
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

- No need to use `#[cfg(test)]` here as the compiler treats the `tests` directory specially.
- `cargo test --test integration_test` - runs just the specified test file

## Submodules in Integration Tests

- File structure for importing things doesn't work in the same way in the `tests` folder.
  - It is if each folder is it's own crate.
- You should store shared test helper functions and such in `tests/some_folder/mod.rs` e.g. `tests/common/mod.rs`, so that it doesn't get run as a test suite when you run `cargo test` and muddy the test output.

## Integration Tests for Binary Crates

- You can't test `main.rs`, only stuff in `lib.rs`. Keep as much code in `lib.rs` as possible.


