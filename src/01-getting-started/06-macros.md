# Macros

Ferrite provides lightweight macros to make it easier to write Ferrite programs.
For most of the macros, there is a corresponding Ferrite function of the same
name available. For instance, the `receive_value_from!` macro desugars into
a function call to `receive_value_from`.

The main purpose for the macros in Ferrite is to reduce various boilerplate
code. For instance, to allow async/await code inside Ferrite programs,
Ferrite requires continuation closures to be passed as
[async closures](https://github.com/rust-lang/rfcs/blob/master/text/2394-async_await.md#async--closures).

## Hello World Without Macros

The macros provided by Ferrite is completely optional, and it is feasible
to write readable Ferrite programs without the help of macros. As an example,
the previous hello world example in the [previous chapter](./03-communication.md)
can be rewritten without macros as follows:

```rust
{{#include ../../code/src/hello_3.rs:hello_3}}
```

Compared to the [original code](./03-communication.md#full-program), we can
see that macro calls such as `receive_value_from! ( provider, greeting => { ... })`
are desugared to `receive_value_from ( provider, move | greeting | async move { ... }`.
In other words, the body of our continuation is passed as an async closure
to the `receive_value_from` function.

## Async Closures

The verbose syntax of `move | | async move { ... }` is required here, because the
new syntax for async closure, `async move | | { ... }`, is currently still being
stabilized in Rust. Nevertheless, if we use Rust nightly and
enables `#![feature(async_closure)]`, we can also rewrite our macro-less
Ferrite program as follows:

```rust
{{#include ../../code/src/hello_4.rs:hello_4}}
```

## Macros in Complex Ferrite Programs

For the case of our hello world example, the difference between the version
with macros and the version without macros might not look significant.
However as we write more complex Ferrite programs, the savings of
having to repeatedly typing keywords like `async move` can become
significant.

Nevertheless, it is also important to understand the desugared function
constructs behind the macro calls in Ferrite. With a good understanding
of the desugared constructs, it can be much easier to debug a Ferrite
program when we encounter any type error when writing Ferrite programs.

In upcoming chapters, when a new Ferrite construct is introduced, we will
cover both the macro version of the construct, as well as the plain
function version of the construct.
