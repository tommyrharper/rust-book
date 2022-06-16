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


