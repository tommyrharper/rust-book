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
    }
}
```
