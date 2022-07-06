# `RefCell<T>` and the Interior Mutability Pattern

- *Interior Mutability* is a design pattern in Rust.
- It allows you to mutate data even where there are immutable references to that data.
- This pattern uses `unsafe` code.
- The `unsafe` code is wrapped in a safe API.

## Enforcing Borrowing Rules at Runtime with `RefCell<T>`

Unlike `Rc<T>`, `RefCell<T>` represents single ownership over the data it hold.
However:
- With `RefCell<T>` the borrowing rules are enforced at *runtime* (instead of usually at compile time).
- If you break these rules the program will panic and exit.
- This means certain memory safe scenarios are allowed that would be rejected normally.
- A famous example is *the Halting Problem*.

`RefCell<T>` is only used for single-threaded code.

**Recap of reasons to choose which Smart Pointer:**
- Owners:
  - `Rc<T>` enables multiple owners of the same data.
  - `Box<T>` and `RefCell<T>` Have single owners.
- Borrows:
  - `Box<T>` allows immutable or mutable borrows at compile time
  - `Rc<T>` allows only immutable borrows checked at compile time
  - `RefCell<T>` allows immutable or mutable borrows checked a runtime.
    - This means you can mutate the value inside `RefCell<T>` even though the `RefCell<T>` itself is immutable.

## Interior Mutability: A Mutable Borrow to an Immutable Value

```rust
// this won't compile
fn main() {
    let x = 5;
    let y = &mut x;
}
```

## A Use Case for Interior Mutability: Mock Objects

A *test double* is the general programming concept for a type used in place of another type during testing.
*Mock objects* are specific types of test doubles that record what happens during a test so you can assert that the correct actions took place.

Rust doesn't have objects in the same sense as other languages.
Rust doesn't have mock object functionality built into the standard library.

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```

- This will fail due to the borrow checker.

Here is the fix using `RefCell<T>`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

- You still must follow the borrow checker rules, i.e. this would panic:

```rust
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        // double mutable borrow not allowed
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}
```

- This could only be caught in production!
- Also there is a small performance overhead for keeping track of borrows at runtime rather than at compile time.

## Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`

- An `Rc<RefCell<T>>` can have multiple owners and be mutated.

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

When run we can see that `value` has been changed for all data structures:

```bash
$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
    Finished dev [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/cons-list`
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
```