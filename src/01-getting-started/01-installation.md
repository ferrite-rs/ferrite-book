# Installation

Ferrite is an open source project with the source code available at
[GitHub](https://github.com/maybevoid/ferrite). Ferrite is also published at
[crates.io](https://crates.io/crates/ferrite-session) as the Cargo
crate `ferrite-session`.

To start using Ferrite, simply
[create a new Cargo project](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
and add `ferrite-session` as a dependency in `Cargo.toml`.
To help with defining asynchronous Rust programs, we also recommend
adding [async-std](https://docs.rs/async-std/) as a dependency in your Rust project.

```
[dependencies]
...
ferrite-session = "0.1.0"
async-std = { version = "1.7.0", features = ["attributes"] }
...
```

To use the constructs provided by Ferrite, import everything from the
`ferrite_session` module:

```rust
use ferrite_session::*;
```

Next, we will learn how to use Ferrite to write a simple hello world program.