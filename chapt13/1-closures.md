# Closures

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```

## Capturing the environment with closures

3 ways (directly map to taking ownership, borrowing mutably and borrowing immutably):
- `FnOnce` -> takes ownership of captured variables. That is why it is called `Once`, because this type of closure can only be called once, as once it has been called its captured variables are deallocated from memory. These variables taken from the enclosing scope are known as the the closures `environment`.
- `FnMut` -> Can change the environment as it mutably borrows
- `Fn` -> Borrows values from the environment immutably.

All closures implement `FnOnce` as they can all be called at least once.
Closures that don't move the captured variables also implement `FnMut`.
Closures that don't need mutable access to captured variables implement `Fn`.

`let equal_to_x = |z| z == x;` is `Fn` as it borrows `x` immutably.

If you want to force the closure take ownership of a value you can use the `move` keyword before the parameter list:

```rust
let equal_to_x = move |z| z == x;
```

This is useful for passing a closure to a new thread to move the data so it's owned by the new thread.

