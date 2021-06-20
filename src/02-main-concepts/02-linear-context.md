# Linear Context

A Ferrite program offers exactly one protocol, as denoted by the type `Session<A>`, with
`A` being the provided protocol. For example, when we define our
[`hello_provider`](../01-getting-started/02-hello-world.md) program
that offers `SendValue<String, End>`, the type of the program is `Session<SendValue<String, End>>`.
When we implement `hello_provider` as `send_value("Hello World!".to_string(), terminate())`,
we are implicitly sending the string value `"Hello World!"` to the _offered_ channel
of the protocol `SendValue<String, End>`.

Although a Ferrite program can only have one offered channel, it can also interact with
other Ferrite programs and act as the _client_ of the offered channels. Since there
can be more than one client channels available, we need to identify the different
channels available using _channel variables_.
So in the case of [`hello_client`](../01-getting-started/03-communication.md),
we bind the channel variable `a` to the client channel of protocol
`SendValue<String, End>`, so that it can _receive_ a string value from the provider.

```rust, noplaypen
{{#include ../../code/src/hello_2.rs:hello_client}}
```

## Type Level List

Ferrite tracks the linear usage of channel variables in a _linear context_.
A linear context is encoded as a Rust type containing zero or more protocols,
in the form of a _type level list_. We can use the `HList!` macro to define
a list of Rust types. For example:

  - `HList![]` is an _empty_ type level list.
  - `HList![String]` is a type level list with just one element, `String`.
  - `HList![String, i32, bool]` is a type level list containing 3 Rust types: `String`, `i32`, and `bool`.
  - `HList![SendValue<String, End>, ReceiveValue<i32, End>, End]` is a type level list containing 3
    Ferrite protocols: `SendValue<String, End>`, `ReceiveValue<i32, End>`, and `End`.

Behind the scene, the `HList!` macro desugars a type level list into nested tuples
of the form `(A0, (A1, (A2, (..., (An, ())))))`. This is similar to how linked lists
are constructed in Lisp. We start by treating the unit type `()` as the empty list.
To prepend an element `A` to the front of another list `R`, we use the tuple constructor
`(,)` to form the new list `(A, R)`, with `A` being the _head_ of the list and `R`
being the _tail_ of the list. So for example:


  - `HList![]` desugars to `()`.
  - `HList![String]` desugars to `(String, ())`.
  - `HList![String, i32, bool]` desugars to `(String, (i32, (bool, ())))`.
  - `HList![SendValue<String, End>, ReceiveValue<i32, End>, End]` desugars to
    `(SendValue<String, End>, (ReceiveValue<i32, End>, (End, ())))`.

When we encounter compile errors while writing Ferrite programs, we can often
see desugared type level lists shown in the error messages. To understand the
error messages, we just need to recognize the patterns and think of them
as being the same as the pretty printed `HList![...]` elements.


## The `Context` trait

Ferrite requires linear contexts to implement the `Context` trait. This is
automatically implemented for a type level list, if all its elements implement
the `Protocol` trait. So for example:

  - `HList![]` is a valid linear context, and it is an _empty_ context.
  - `HList![SendValue<String, End>]` is a linear context with one protocol, `SendValue<String, End>`.
  - `HList![SendValue<String, End>, ReceiveValue<i32, End>, End]` is a linear context,
    because all 3 elements are valid Ferrite protocols.
  - `HList![String, i32, bool]` is not a linear context, because none of them are valid
    Ferrite protocols.
  - `HList![SendValue<String, End>, String]` is not a linear context, because the
    second element `String` is not a Ferrite protocol.
  - `HList![SendValue<String, End>, ReceiveValue<i32, i32>]` is not a linear context,
    because the second element `ReceiveValue<i32, i32>` is not a valid Ferrite protocol.

All Ferrite program has an associated linear context, and a program usually starts with an
empty linear context `HList![]`. For the case of `hello_client`,
in the beginning it also starts with the empty context. When we use `receive_channel`
inside `hello_client`, a new channel is then added to the linear context in the continuation.
As a result, the linear context in the continuation after `receive_channel` becomes
`HList![SendValue<String, End>]`, with the channel variable `a` referring to the first
channel `SendValue<String, End>`.

## Empty Slots

When a Ferrite program has a non-empty linear context, it would have to fully
consume all linear channels before it can finally terminate. For the case
of `hello_client`, this would mean that the linear context `HList![SendValue<String, End>]`
has to be fully consumed. We do that by first receiving the value using
`receive_value_from(a, ...)`. In the continuation, the linear context would be
updated to become `HList![End]` after the value is received. At this point
we would then wait for the channel to terminate using `wait(a, ...)`.

After the channel `a` is terminated, the original position of the channel
in the linear context is updated with a special _empty slot_ called `Empty`.
This is used to indicate that the channel has been fully consumed, and
it may be safe for the program to terminate. So in the continuation
after `wait(a, ...)`, the linear context is updated to become `HList![Empty]`.

Compared to `End`, `Empty` is not a protocol. So we cannot for example define a
Ferrite program  that offers `Empty`. However `Empty` is treated as a valid
element in a linear context. So a type level list like
`HList![Empty, SendValue<String, End>]` also implements `Context`.

## Empty Context

When we call `terminate()`, Ferrite checks whether a linear context is empty
using the `EmptyContext` trait. This trait is implemented if the linear context
is an empty list `HList![]`, or if all elements in the linear context is `Empty`.
So `HList![]`, `HList![Empty]`, `HList![Empty, Empty]`, and so on all implement
`EmptyContext`.

When we try to call `terminate()` with a non-empty linear context,
we will get a compile error stating that the type `HList![...]` does not
implement `EmptyContext`. When encountering such error, we can inspect
the list and see which element is not empty, and update our Ferrite
program to fully consume the channel accordingly.
