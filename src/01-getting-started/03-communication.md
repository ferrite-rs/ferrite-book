# Client Communication

In the previous chapter, we defined a hello provider and then run it
immediately using `run_session_with_result`. However in practical applications,
our Ferrite programs will typically have more complex session types, and
we would want to run multiple programs in parallel that communicate with
each others.

To demonstrate that, instead of running `hello_provider` directly, we can
define a `hello_client` that _communicates_ with a provider of the `Hello`
protocol, and then link it with `hello_provider`.

First, we define `hello_client` to have the following type:

```rust
let hello_client :
  Session <
    ReceiveChannel < Hello, End >
  > = ...
```

Our `hello_client` has the Rust type `Session < ReceiveChannel < Hello, End > >`,
indicating that it is a Ferrite program offering the session type
`ReceiveChannel < Hello, End >`. Here we encounter a new session type in the form
`ReceiveChannel < A, B >`, which is used to _receive a channel_ of session type
`A` offered by some provider, and then continue offering session type `B`.

## Channels

Ferrite allows sending and receiving of _channels_, which represent the client
end point to communicate with a provider that offers the channel's session type.
At high level, we can think of receiving a channel using `ReceiveChannel < A, B >`
is similar to plain Rust functions with function traits like `FnOnce (A) -> B`.
In other words, `ReceiveChannel` is the equivalent of function types in Ferrite.

There is however one important distinction of `ReceiveChannel` from regular
Rust functions. That is channels received from `ReceiveChannel` must be
used _linearly_, i.e. they cannot be ignored or dropped.
We will revisit this property in later chapters to see why linearity is
important, and why bare Rust code is not sufficient to support linearity.


Now back to our `hello_client` example. The session type
`ReceiveChannel < Hello, End >` indicates that `hello_client` is _receiving_
a channel of session type `Hello`, and then terminates with `End`.
To implement such a session type, we can implement `hello_client` as follows:

```rust
{{#include ../../code/src/hello_2.rs:hello_client}}
```

Our `hello_client` body looks slightly more complicated than `hello_provider`.
To understand what's going on, we will go through each line of
`hello_client`'s body. Starting with `receive_channel!`:

```rust
receive_channel! ( provider => {
  ...
})
```

To match the final session type `ReceiveChannel < Hello, End >` that is
offered by `hello_client`, we use `receive_channel!` to receive a
channel of session type `Hello`, and then binds it to the _channel variable_
`provider`. This is similar to a Rust function accepting an argument
and bind it to a variable.

Inside the continuation `...`, since we have received the `Hello` channel
already, we will continue to offer the session type `End`.  To do that,
we just need to eventually terminate `hello_client`. However we cannot
terminate `hello_client` just yet, because the channel variable `provider`
is _linear_, and we must fully consume it before we can terminate.

Recall that `Hello` is a type alias, so the actual session type of
the channel variable `provider` is `SendValue < String, End >`.
But instead of having to offer that, we are acting as the _client_
to consume the session type `SendValue < String, End >`. Since the provider
is expected to send a `String` value, as a client we are expected to
_receive_ a `String` value from the provider. We can do that using
`receive_value_from!`:

```rust
receive_value_from! ( provider, greeting => {
  println! ( "Received greetings from provider: {}", greeting );
  ...
})
```

We use `receive_value_from!` to receive a value sent from the `provider`
channel, and then bind the received `String` value to the Rust variable
`greeting`. We then print out the value of `greeting` using `println!`.
Following that, in the continuation `...`, the session type
of `provider` _changes_ from `SendValue < String, End >` to
become `End`.

Unlike regular Rust variables, each time we interacts with a channel
variable, the session type of the channel variable is _updated_
to its continuation session type. Since Ferrite channels are linear,
we have to continuously interact with the channel until it is
fully terminated.

After calling `receive_value_from!`, we have the channel variable
`provider` with session type `End`, and we need to offer the session
type `End` by terminating. But we can't terminate just yet, because
`End` simply indicates that the provider will eventually terminates,
but may not yet been terminated. Hence we would first have to wait
for `provider` to terminate using `wait!`:

```rust
wait! ( provider, terminate! () )
```

We use `wait!` to wait for the provider on the other side of a
channel to terminate. After that, the `provider` channel is discarded,
and we don't have anymore unused channel variable. With that, we
can finally terminate our program using `terminate!()`.

## Linking Provider With Client

We have now defined a `hello_client` program that can accept channel
from any provider offering the session type `Hello`.
In theory, we can call `hello_client` with any provider that offers
`Hello`, not just `hello_provider`.

To link `hello_client` specifically with `hello_provider`, we have to
explicitly ask Ferrite to perform the linking. This can be done
using `apply_channel`:

```rust
{{#include ../../code/src/hello_2.rs:apply_channel}}
```

The `apply_channel` construct is provided by Ferrite to link a
client Ferrite program of session type `ReceiveChannel < A, B >`
with a provider Ferrite program of session type `A`,
resulting in a new Ferrite program of session type `B`
as the result.

We can think of the form `apply_channel(f, x)` as being similar similar to
regular function application in the form `f(x)`. With `f` having the Rust type
`FnOnce(A) -> B` and `x` having the Rust type `A`, the result type of applying
`x` to `func` would be `B`.

## Run Session

After applying `hello_provider` to `hello_client`, we now have a single `main`
program. When `main` is executed, it will run both `hello_provider` and
`hello_client` in parallel, and establish a communication channel between
the two processes.

Since `main` has the session type `End`, we can use the Ferrite construct
`run_session` to run the program:

```rust
{{#include ../../code/src/hello_2.rs:run_session}}
```

`run_session` accepts any Ferrite program offering the session type `End`,
executes the program, and wait for it to terminate. `run_session` is
similar to `run_session_with_result`, other than it does not expect
the Ferrite program to send back any Rust value as the result.


## Full Program

Putting everything together, we now have our second hello world program
that is made of a `hello_provider` and a `hello_client` communicating
with each others.

```rust
{{#include ../../code/src/hello_2.rs:hello_2}}
```