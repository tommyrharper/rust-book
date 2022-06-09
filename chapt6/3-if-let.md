# The if let control flow

- Imagine you only want to do something if a value is `Some` value:

```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
```

We can write this in a shorter way using `if let`:

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

- This works the same way as a `match` statement except it is just for one option and doesn't have to be exhaustive.
- If let is syntactic sugar for a match statement only has one branch and ignores all others.


- Imagine you want to print the quarter coins state and print all non-quarters:

Using match:
```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

Using if let:
```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("The state is: {:?}!", state);
} else {
    count += 1;
}
```
