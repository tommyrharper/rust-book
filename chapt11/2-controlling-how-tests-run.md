# Controlling How Tests Are Run

- `cargo test` - default behaviour runs the tests in parallel and capture output generated during test runs, preventing that output from being displayed and making it easier to read the output related to the test results.
- `cargo test --help` - view the options you can use with `cargo test`
- `cargo test -- --help` - display the options you can use after the `--` operator

Some command line options go to `cargo test` and some go to the results test binary. To separate these two types of arguments, you list the `cargo test` arguments followed by `--` and then the ones that go to the test binary.

## Running Tests in Parallel or Consecutively

- Default is a new thread for each test
- `cargo test -- --test-threads=1` use one thread - don't use any parallelism.
  - This takes longer, but you tests won't interfere with each other if there is shared state

## Showing Function Output

- By default all output for a passing test is captured by the test library.
- Failing tests do print output to the console.
- `cargo test -- --show-output` - show all output even in passing tests.

## Running tests by name

- `cargo test test_name` - run the test with that name or the subset of tests that contain that string in their name

## Ignoring Some Tests Unless Specifically Requested

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

- `cargo test -- --ignored` to run the `expensive_test`
- `cargo test -- --include-ignored` run all tests.