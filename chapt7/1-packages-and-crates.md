# Packages and Crates

- A *package* is one or more crates that provide a set of functionality.
  - Contains a `Cargo.toml` file that defines how to build the crates

- A *crate* can be a binary crate or a library crate
- *Binary crates* are compiled to an executable you can run such as a cli interface or a server.
  - Must have a function called `main` that defines what happens when the executable runs.

- *Library crates* don't have a `main` function and they don't compile to an executable.
  - They define functionality to be shared by multiple projects.
  - e.g. the `rand` crate used in chapt 2 to create a random number

- The *crate root* is a source file that the rust compiler starts from and makes up the root module of your crate.

- A package can contain at most one library crate, but as many binary crates as you want.
  - It must contain at least one crate.

To create a new package:
```
cargo new my-project
```

- `src/main.rs` is the crate root of a binary crate.
- If the package contains `src/lib.rs`, the package has a library crate and that is the crate root.
- If you have `src/main.rs` and `src/lib.rs` your package has two crates, a binary crate and a library crate.
- To add extra binary crates add them in the `src/bin` directory, each file will be a separate binary crate.
