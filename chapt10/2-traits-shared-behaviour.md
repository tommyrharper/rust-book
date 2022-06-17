# Traits - Defining Shared Behaviour

- Defines functionality that a specific type can share with other types.
- Allows us to define shared behaviour in an abstract way.
- We can use *trait bounds* to specify that a generic type can be any type has certain behaviour.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

## Implementing a Trait on a type

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

- Used elsewhere in the code, you have to import the trait:

```rust
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```

- You can't implement external traits on external types.
- Hence you can't implement the `Display` trait on on `Vec<T>` because they are both defined in the standard library and aren't local to the `aggregator` crate.
  - This is part of a property called *coherence*
    - More specifically the *orphan rule*
      - Because the parent is not present
      - This ensure other people's code cannot break your own
      - Imagine if the parent implemented a trait you implemented on the child, which would rust use?

## Default Implementations

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {}
```

- Default implementations can call other methods on the same trait:

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

- You can't call the default implementation from an overriding implementation.

## Trait as Parameters

- You can write functions that accept any argument that implements a specific trait:
```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

## Trait Bound Syntax

- This is a more verbose way of saying the above:
```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
- This more verbose syntax can be used to express more complex situations:

```rust
// both parameters could have different types:
pub fn notify(item1: &impl Summary, item2: &impl Summary) {..}
// to force both params to be the same type:
pub fn notify<T: Summary>(item1: &T, item2: &T) {..}
```

## Specifying multiple trait bounds with the `+` syntax

- Combine multiple traits:
```rust
pub fn notify(item: &(impl Summary + Display)) {..}
// or
pub fn notify<T: Summary + Display>(item: &T) {..}
```

## Clearer trait bounds with where clauses

- Instead of this:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {..}
```
- We can do this:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{..}
```

This is easier to read.

## Returning types that implement traits

- You can do this:
```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

Unfortunately this does not work:
```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

To see how go to 'Using Trait Objects That Allow for Values of Different Types' in chapt 17.

## Fixing the `largest` Function with Trait Bounds

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

## Using Trait Bounds to Conditionally Implement Methods

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// this is conditionally implemented based on if T has the associated traits implemented
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```


- Implementations on a trait that satisfy the trait bounds are called *blanket implementations* are are used extensively in the standard library.
- E.g. the `ToString` trait is implemented on any type that implements the `Display` trait.
```rust
impl<T: Display> ToString for T {
    // --snip--
}
```
- Hence you can invoke `to_string` on any type that implements the `Display` trait, e.g. on integers:

```rust
let s = 3.to_string();
```

