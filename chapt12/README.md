# Command Line I/O Project

- we are making `grep` - **g**lobally search a **r**egular **e**xpression and **p**rint.
- print result to `stderr` - the standard error console stream, rather than `stdout` - standard output
- there already a super fast rust version of `grep` - `ripgrep`]

## Accepting Command Line Arguments

We want to be able to run `cargo run searchstring example-filename.txt`

## My API

```zsh
cargo run to poem.txt # Search for 'to' in poem.txt
cargo run to poem.txt --insensitive # Case insensitive search
cargo run to poem.txt > output.txt # Pipe stdout into output.txt file
export CASE_INSENSITIVE=1 # Set all queries to case insensitive for the length of the session
unset CASE_INSENSITIVE # Return all queries to case sensitive
printenv # see all environment variables
set # see all environment variables
printenv CASE_INSENSITIVE # see if CASE_INSENSITIVE is set
```