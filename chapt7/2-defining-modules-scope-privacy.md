# Defining Modules to control scope and privacy

- `use` to bring things into scope
- `pub` to make them public


## Two types of paths

- absolute paths
  - Starts from crate root, for own crate this is literally `crate`, for other crates it is the crate name.
- relative paths
  - Starts from the current module and uses `self`, `super` or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).

