# Chapt 4 - Ownership

## Stack vs Heap

- Stack is faster but values must have a fixed size
- Heap is slower but can handle variable sizes. Uses pointers.

## Ownership Rules

- Each value in Rust has a variable that's called it's *owner*
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

## Variable Scope

- a variable is block scoped
```rust
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

## The String type


