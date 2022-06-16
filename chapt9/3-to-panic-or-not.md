# To panic or not to panic

- Returning `Result` is generally better because they calling code can decide how to handle a failure.
- In examples, prototypes and tests you can just call panic.

- Also if you know your code won't fail, its cool to use panic/unwrap:

```rust
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1".parse().unwrap();
```

## Guidelines for error handling

- You should panic when your code will be in a bad state (some unexpected/missing values come through), your code contract is broken.
- When failure is expected, return a `Result`.

## Creating custom types for validation

- Lets add a custom type for validating inputs are between 1 and 100 for our guessing game we made earlier:

```rust
loop {
    // --snip--

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
        // --snip--
    }
}
```
- ^^^ This is non-ideal and not reusable.
- Lets improve it:

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```


