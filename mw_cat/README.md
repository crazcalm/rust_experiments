# mw_cat
## Why this exists
I wanted to get a better handle on how to read stdin with reference to how unix pipes works.

For example:
```
echo "hi" | cargo run
```
The application has zero arguments. The output from the echo is treated as stdin.

With this in mind, I also wanted a handle on how applications switch from processing stdin to processing arguments.

For example:
```
cargo run src/main.rs Cargo.toml
```