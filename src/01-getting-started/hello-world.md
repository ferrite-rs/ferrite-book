# Hello World

## Dependencies

```toml
[dependencies]
ferrite = { git = "https://github.com/maybevoid/ferrite" }
async-std = { version = "1.6.5", features = ["attributes"] }
```

## Hello World Provider

```rust
{{#include ../../code/src/hello.rs:hello_1}}
}
```

## Hello World Provider and Client

```rust
{{#include ../../code/src/hello.rs:hello_2}}
```