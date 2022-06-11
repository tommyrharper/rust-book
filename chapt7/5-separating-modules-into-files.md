# Separating Modules into Different Files

- You only need to load the contents of a file using `mod` declaration once somewhere in your module tree. Once the compiler knows the file is part of the project (and knows where in the module tree the code resides due to where the `mod` statement is placed), other files should refer to the code in that file using a path to where it was declared.
- The directories and files closely match the module tree.

## Alternate File Paths

There is also an old way of doing things that is still supported.

- For a module named `front_of_house` declared in the crate root, the compiler will look for the modules code in:
  - New way: `src/front_of_house.rs`
  - Old way: `src/front_of_house/mod.rs`

- For a module named `hosting` that is the submodule of `front_of_house`, the compiler will look for the module's code in:
  - New way: `src/front_of_house/hosting.rs`
  - Old way: `src/front_of_house/hosting/mod.rs`

If you use both for the same module, you'll get a compiler error. Try to be consistent.
