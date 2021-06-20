# Installation

Ferrite is an open source project with the source code available at
[GitHub](https://github.com/maybevoid/ferrite). Ferrite is also published at
[crates.io](https://crates.io/crates/ferrite-session) as the Cargo
crate `ferrite-session`.

To start using Ferrite, simply
[create a new Cargo project](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
and add `ferrite-session` as a dependency in `Cargo.toml`.
Ferrite uses [tokio](https://docs.rs/async-std/) to spawn async tasks, so you should
add that as a dependency as well.

```
[dependencies]
tokio = "1.6.1"
ferrite-session = "0.1.4"
...
```

To use the constructs provided by Ferrite, import everything from the `ferrite_session::prelude` module. You'd also need to provide a `tokio` runtime for Ferrite to spawn its async tasks. This can be done by adding the `#[tokio::main]` attribute to your main function.

```rust, noplaypen
use ferrite_session::prelude::*;

#[tokio::main]
async fn main() { ... }
```

Next, we will learn how to use Ferrite to write a simple hello world program.
