# Type Errors

The way we write a Ferrite program is quite different from how we typically
write a Rust program. This is because Ferrite makes use of Rust's type system
to provide additional safeguard on how session type channels can be used.

We can take a quick look of some error messages raised when our Ferrite
programs are written incorrectly.

## An Incorrect Hello Provider

Let's consider the `hello_provider` program that we have written in the
previous chapter:

```rust
{{#include ../../code/src/hello_2.rs:hello_provider}}
```

According to the `Hello` protocol, `SendValue<String, End>`,
`hello_provider` is supposed to send a string value before terminating.
But what if we implement `hello_provider` such that it terminates
immediately without sending a string?


```rust
let hello_provider_incorrect: Session<Hello> = terminate();
```

In this case, if we try to compile our program, we would get a compile
error from Rust showing a message similar to as follows:

```
error[E0308]: mismatched types
 |   let hello_provider_incorrect: Session<Hello> = terminate();
 |                                 --------------   ^^^^^^^^^^^ expected struct `ferrite_session::prelude::SendValue`, found struct `ferrite_session::prelude::End`
 |                                 |
 |                                 expected due to this
 |
 = note: expected struct `PartialSession<(), ferrite_session::prelude::SendValue<String, ferrite_session::prelude::End>>`
            found struct `PartialSession<_, ferrite_session::prelude::End>`
```

The error message above has been slightly prettified to make it more readable,
but the content remains the same. First of all, we see that the error is about
type mismatch between a `PartialSession` type, which we will cover in a
[later chapter](../03-main-concepts/03-partial-sessions.md).

Inside the second type parameter of `PartialSession`, notice that
there is a mismatch between the expected session type
`SendValue < String, End >`, and the actual session type `End`. From that,
we can deduce that `hello_provider_incorrect` has violated the `Hello`
protocol, by instead offering the protocol `End`.

## An Incorrect Hello Client

The error message we get from `hello_provider_incorrect` is relatively simple,
as the type mismatch is on the offered channel. On the other hand, recall that
`hello_client` deals with _two_ channels: one with the receiver end of `Hello`,
and one that offers the session type `End`.


```rust
{{#include ../../code/src/hello_2.rs:hello_client}}
```

Based on the protocol specification, it looks like `hello_client` should be able
to receive a `Hello` channel, ignores the channel, and proceed to terminate
immediately. So let's try implementing a new client to do just that:

```rust
let hello_client_incorrect: Session<ReceiveChannel<Hello, End>> =
  receive_channel(|provider| {
    terminate()
  });
```

In `hello_client_incorrect`, the `provider` channel is ignored after we have received
it, and we proceed to call `terminate` immediately. If we try to build this,
we would instead get the following compile error:

```
error[E0277]: the trait bound `(ferrite_session::prelude::SendValue<String, ferrite_session::prelude::End>, ()): ferrite_session::internal::base::EmptyContext` is not satisfied
 |
 |     terminate()
 |     ^^^^^^^^^ the trait `ferrite_session::internal::base::EmptyContext` is not implemented for `(ferrite_session::prelude::SendValue<String, ferrite_session::prelude::End>, ())`
 |
 |
 |   C: EmptyContext,
 |      ------------ required by this bound in `ferrite_session::prelude::terminate`
 |
 = help: the following implementations were found:
           <(ferrite_session::prelude::Empty, R) as ferrite_session::internal::base::EmptyContext>
```

The error can look a bit scary, but it mainly boils down to two constructs:
a new type `(SendValue<String, End>, ())` and a new trait `EmptyContext`.

The type `(SendValue<String, End>, ())` represents the _linear context_ in
`hello_client_incorrect`, when the expression `terminate()` is called. We will
cover the details of linear context in a
[later chapter](../03-main-concepts/02-linear-context.md). For now it is sufficient
to know that it is used to track the `provider` channel we have just received,
and it has the session type `SendValue<String, End>`.

The `EmptyContext` trait is essentially telling us that in order to use the
`terminate()` construct, the current linear context must be empty.
However because we have not yet used up the `provider` channel, the
linear context is thus not empty, and so we cannot terminate just yet.

## Enforcing Linear Usage at Compile Time

The error from `hello_client_incorrect` shows us how Ferrite enforces linear
usage of session type channels at compile time. This can greatly reduce the
chance of us writing incorrect Ferrite programs that just abort in the
middle of communicating with other processes.

The linear usage of Ferrite channels is different from the _affine_ usage
of Rust objects. Consider an equivalent of `hello_client` implemented
using [Rust channels](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html):

```rust
fn hello_client_rust(receiver: Receiver<String>) {
  // drops receiver immediately
}
```

In standard Rust programs, we establish communication between concurrent
processes by explicitly passing the sender or receiver end of a Rust channel
as function argument. However since the Rust channel is just an affine
Rust object, it is possible that one party drops the channel half way,
intentionally or not.

In the simple case of our `Hello` protocol, the dropping of a channel
might not seem critical. But what if the protocol is slightly more complicated?

Let's say we have an `Ping` protocol with the session type
`ReceiveValue<String, SendValue<String, End>>`. This would require the
provider to receive a string value before sending back another string value,
and vice versa for the client. If a client drops the channel without
sending a value to the provider, the provider would potentially wait forever
without receiving the string.

Even if the provider can somehow detect that the client has dropped the
channel half way, it would still have to raise a runtime error to signal
the error on protocol violation. We would then have to write a lot of
tests to make sure such errors never happen in practice. Even so the
tests may not be exhaustive enough to cover all corner cases.

By enforcing the linear usage of channels at compile time, Ferrite helps
eliminate a broad range of communication errors that can happen due to
incorrect implementations. As we will see in later chapters, using Ferrite
we can safely implement complex communication protocols in our Rust programs,
without having to worry about our programs not correctly following
these protocols.
