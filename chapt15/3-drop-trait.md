# Running Code on Cleanup with the `Drop` Trait

- Executes when a value goes out of scope.
- Can be used to release resources like files or network connections.
- When a `Box<T>` is dropped it will deallocate the space on the heap the box points to.

```rust
struct CustomSmartPointer {
    data: String;
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```

## Dropping a Value Early with `std::mem::drop`

- *destructor* is the general programming term for a function that cleans up an instance.
  - This is analogous to a *constructor* which creates an instance.
  - The `drop` function in Rust is one particular *destructor*.

Rust doesn't let us call `drop` explicitly because Rust would still automatically call `drop` once the value exits its scope causing a *double free* error.

Hence if you want to force a value to be cleaned up early you must use the `std::mem::drop` function.

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```
