# Protocols

Ferrite uses the term _protocol_ to refer to session types in Rust. All protocol in Ferrite implements the
`Protocol` trait. The name of the protocols are defined based on the behavior of the provider.
Most of the Ferrite protocols also contain _continuation_ protocols, which describe the protocol that
follows after the current protocol have been executed.

We have seen an example protocol in the [previous section](../01-getting-started/02-hello-world.html),
`SendValue<String, End>`. This means that the type `SendValue<String, End>` implements the `Protocol` trait.
This is so because `End` is a protocol representing termination, and the type `SendValue<T, A>` is a protocol
if the _continuation_ protocol `A`, which is `End` in this case, is a protocol. The type `SendValue<String, End>`
describes a protocol which the provider sends a `String` value and then terminates with `End`. On the other end,
this behavior is inversed, with the client _receives_ the `String` value and then _waits_ for the provider to terminate.

For a Rust type to be a Ferrite protocol. It has to follow certain conditions. For example, while we can define a Rust type
like `SendValue<String, i32>`, it is not considered by Ferrite to be a protocol. This is because the continuation
type `i32` does not implement `Protocol`. Therefore `SendValue<String, i32>` also does not implement `Protocol`.

Some protocols also have additional constraints that need to be satisfied. For example, the protocol `SendValue<T, A>`
also requires the Rust value to be send, `T`, to implement [`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html) and `'static`.
This means that we can only send Rust values that can be safely transferred across threads, and do not contain other local lifetimes.

Using Ferrite, we can define complex protocols that have many layers of nested continuations.
For example, the protocol `SendValue<String, ReceiveValue<i32, SendValue<bool, End>>>`
describes a session type which the provider first sends a `String` value, then receives
an `i32` value, then sends a `bool` value, before finally terminating.

## Overview of Linear Protocols in Ferrite

We will cover in detail each protocol in the upcoming chapters. Below shows a summary of protocols available in Ferrite:

| Protocol    | Provider Description           |
|-------------|--------------------------------|
| [`End`](../03-basic-constructs/01-termination.md)       | Termination   |
| [`SendValue<T, A>`](../03-basic-constructs/02-send-receive-values.md)  | Send a Rust value of type `T`, and continue as session type `A` |
| [`ReceiveValue<T, A>`](../03-basic-constructs/02-send-receive-values.md)  | Receive a Rust value of type `T`, and continue as session type `A` |
| [`SendChannel<A, B>`](../03-basic-constructs/03-send-receive-channels.md) | Send a channel of session type `A`, and continue as session type `B` |
| [`ReceiveChannel<A, B>`](../03-basic-constructs/03-send-receive-channels.md) | Receive a channel of session type `A`, and continue as session type `B` |
| [`ExternalChoice<Choice>`](../03-basic-constructs/04-external-choice.md) | Provide the client with choices defined in `Choice` |
| [`InternalChoice<Choice>`](../03-basic-constructs/05-internal-choice.md) | Offer a particular choice defined in `Choice` |
| [`Rec<F>`](../04-advanced-constructs/01-recursive-session-types.md) | Recursive session type, with `F` being a session type containing recursion markers such as `Z`. |
