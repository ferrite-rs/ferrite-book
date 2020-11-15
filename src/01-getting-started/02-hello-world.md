# Hello World

In this chapter we will look at two simple programs that implement
hello world in Ferrite.

## Hello Protocol

A session type, a.k.a. a protocol, describes a communication protocol between
two parties: a _provider_ and a _client_. The provider _offers_ a service
as described by the session type, and the client _consumes_ the provided
service in a linear fashion.

In this example, we will define a simple `Hello` protocol that has
the session type `SendValue < String, End >`:

```rust
type Hello = SendValue < String, End >;
```

Our `Hello` protocol pretty much self describe the communication protocol:
The provider would send a Rust `String` value and then terminates. Conversely,
a client for `Hello` would _receive_ a Rust `String` value, and then _waits_
for the provider to terminate.

The type `SendValue < T, A >` defines a session type that sends a Rust value
`T`, and then continues with the _continuation_ session type `A`. The type
`End` defines a session type that simply terminates. When we combine both
`SendValue` and `End` to get `SendValue < String, End >`, we are effectively
defining a session type that sends a Rust value of type `String`, and then
_continues_ as session type `End`, which happens to simply terminates.

## Hello World Provider

We first look at how a provider for the `Hello` protocol can be implemented:

```rust
{{#include ../../code/src/hello.rs:hello_provider}}
```

In the above example, we define a variable named `hello_provider`
with the Rust type `Session < Hello >`. The Rust type `Session < A >` denotes
a Ferrite program that is providing a session type `A`. In this case,
`hello_provider` is a Ferrite program that provides the `Hello` protocol.

In the body of `hello_provider`, we use the Ferrite macros `send_value!` and
`terminate!` to build up our Ferrite program. According to the `Hello` protocol,
the first step `hello_provider` needs to do is to send a `String` value.
To do that, we create a Rust string `"Hello World!".to_string()`, and then send
it by calling `send_value! ( "Hello World!".to_string(), ... )`.

Other than the `"Hello World!"` string in the first argument, `send_value!` also
expects a _second_ argument, which is the _continuation_ after our string value
is sent. In our case, The continuation session type of `SendValue < String, End >`
is `End`. As such, there is nothing left to do other than terminating the Ferrite
program, which we can do it by calling `terminate!()`.

## Run Session

Up to this point, we have only defined a Ferrite program named `hello_provider`,
but we have not yet execute the program. To run it, we would typically need to
pair it with a client that _consumes_ the offered protocol `Hello`. However
Ferrite provides a special case for Ferrite programs that offer the session types
`SendValue < T, End >`. So we can run our `hello_provider` by calling
`run_session_with_result`:

```rust
{{#include ../../code/src/hello.rs:run_session_with_result}}
```

Ferrite provides `run_session_with_result` as a default way of handling
Ferrite programs offering the session type `SendValue < T, End >`,
because they are trivial to handle.
This can be done by receiving the Rust value sent from the provider,
waits for the provider to terminate, and then returns to the caller.
The function is an async function, so we have to use the `.await`
syntax to wait for Ferrite to run the program and return the results.

After getting the result back, we can print the received string using
`println!`, and we can expect `"Hello World!"` to be printed at this point.

## Full Hello World Program

Putting everything together, our first hello world program is written as follows:

```rust
{{#include ../../code/src/hello.rs:hello_1}}
```

Our Rust program defines an async `main` function using the `#[async_std::main]`
attribute provided by `async-std`. Inside the main body, we define our provider
Ferrite program as `hello_provider`, and then immediately run it using
`run_session_with_result`. Finally we get back the result string and print it
to the terminal.
