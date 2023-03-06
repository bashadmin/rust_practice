### Rust setup

Cargo's bin directory activate
source `"$HOME/.cargo/env"`


MacOS/Linux

add to .bashrc, or .bash_profile
```
nano ~/.bashrc
```
```
export CARGO_HOME=$HOME/.cargo/bin
export PATH=$PATH:$CARGO_HOME
```

run
```
source ~/.cargo/env
```


### Useful cargo commands:
- `cargo new project_dir`
- `cd project_dir`
- `cargo run`
- `cargo build`
- `cargo build --release`
- `cargo check`
- `cargo clean`
- `cargo update`
- `cargo doc --open`
- `cargo test`
- `cargo bench`
- `cargo install`
- `cargo uninstall`
- `cargo search`
- `cargo publish`
- `cargo login`
- `cargo logout`
- `cargo yank`
- `cargo package`
- `cargo fetch`
- `cargo verify-project`
- `cargo init`
- `cargo fix`
- `cargo clippy`
- `cargo tree`
- `cargo install-update -a`
- `cargo outdated`
- `cargo install cargo-edit`
- `cargo add`
- `cargo rm`
- `cargo upgrade`
- `cargo generate-lockfile`
- `cargo install cargo-watch`
- `cargo watch -x run`
- `cargo install cargo-edit`
- `cargo add`


### rust verserved words
- `as`
- `break`
- `const`
- `continue`
- `crate`
- `else`
- `enum`
- `extern`
- `false`
- `fn`
- `for`
- `if`
- `impl`
- `in`
- `let`
  - assign a value to a variable and is immutable by default.
- `loop`
- `match`
- `mod`
- `move`
- `mut`
  - allows a variable to be mutable.
- `pub`
- `ref`
- `return`
- `static`
- `self`
- `Self`
- `struct`
- `super`
- `trait`
- `true`
- `type`
- `unsafe`
- `use`
- `where`
- `while`
- `async`
- `await`
- `dyn`
- `abstract`
- `become`
- `box`
- `do`
- `final`
- `macro`
- `override`
- `priv`
- `typeof`
- `unsized`
- `virtual`
- `yield`
- `try`
- `union`
