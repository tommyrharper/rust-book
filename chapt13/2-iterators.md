# Iterators

- Rust iterators are lazy - they have no effect until consumed.

i.e. this does nothing:
```rust
let v1 = vec![1, 2, 3];
```

i.e.
```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter(); // nothing happens at the point

for val in v1_iter {
    println!("Got: {}", val);
}
```

## The `Iterator` Trait and the `next` Method

The `Iterator` trait looks something like this:
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

The `Iterator` trait requires that the `next` method is implemented and you have the `Item` trait defined. `Item` is returned from the `next` method, wrapped in an `Option` enum as either `Some(x)` or `None`.

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

Note that `v1_iter` is mutable, each invocation of the `next` method updates its value to the next value in the iterators sequence.

The iterators is progressively *consumed* or used up.

- `iter` -> returns immutable references.
- `into_iter` -> returns owned values.
- `iter_mut` -> returns mutable references.

## Methods that Consume the Iterator

Methods that call `next` are called *consuming adaptors*, because calling them uses up the iterator.
e.g. the `sum` method:
```rust
#[test];
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    // v1_iter can no longer be used -> it has been consumed by sum, as sum is a *consuming adaptor*

    assert_eq!(total, 6);
}
```

Here `sum` has taken ownership of the iterator we call it on.

## Methods that Produce other iterators

- *Iterator adaptors* allow you change one iterator into another type of iterator.
- You can chain these together.
- These are lazy, you must call one of the consuming adaptors to get the results.

Hence for the following code:
```rust
let v1: Vec<i32> = vec![1, 2, 3];
v1.iter().map(|x| x + 1);
```
You get a warning:
```bash
$ cargo run
   Compiling iterators v0.1.0 (file:///projects/iterators)
warning: unused `Map` that must be used
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default
  = note: iterators are lazy and do nothing unless consumed

warning: `iterators` (bin "iterators") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/iterators`
```

This is due to lazy evaluation.
You must use the `collect` method to collect the resulting values into a collection data type:

```rust
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
assert_eq!(v2, vec![2, 3, 4]);
```

## Filter

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
```

- Notice the use of `into_iter` -> we are creating an iterator that takes ownership of the vector.
