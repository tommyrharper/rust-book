# Customising Build with Release Profiles

You know about this:
```bash
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
```

Default values for `opt-level` fro dev vs release build:
```md
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

3 is the maximum optimisation level.
If you wanted to override these settings, you can do it in `Cargo.toml`:

```md
[profile.dev]
opt-level = 1
```
