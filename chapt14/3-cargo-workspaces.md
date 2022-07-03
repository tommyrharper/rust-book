# Cargo Workspaces

This allows you to split a large library crate into multiple libraries that are developed in tandem.

## Creating a Workspace

A *workspace* is a set of packages that share the same `Cargo.lock` and output directory.

You create a root file, which will contain all your cargo projects.
Inside the root file you place `Cargo.toml`.

e.g:
```bash
mkdir add
cd add
touch Cargo.toml
cargo new adder
cargo build
```

Then in `Cargo.toml`
```toml
[workspace]

members = [
    "adder",
]
```

The file structure now looks like this:
```
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

All the code compiles into the single `target` at the root folder than contains all the projects.

You can then add extra crates sub-crates as dependencies to one another:

```
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

Then in `Cargo.toml` in `adder`:
```toml
[dependencies]
add_one = { path = "../add_one" }
```

Then you can run a specific crate using `cargo run -p adder`.
Or run specific tests using `cargo test -p add_one` etc.

## Publishing

You will need to publish each crate separately.