# Running Sessions

When we define a Ferrite program with Rust expressions like
`let program: Session<...> = ...`, the program is _not_ executed
until we run it at a later time. Ferrite provides two public functions,
`run_session` and `run_session_with_result` to execute the Ferrite programs.

## `run_session`

```rust
async fn run_session(session: Session<End>)
```

`run_session` is an async function that accepts a Ferrite program of type `Session<End>`,
and blocks until the program has finished executed.

Since `run_session` requires the offered
protocol to be `End`, this means that we cannot use it to execute Ferrite programs that offer
other protocols, such as `ReceiveValue<String, End>`. Intuitively, this is because Ferrite
cannot know in general how to run Ferrite programs offering any protocol.
For instance, how are we supposed to finish run a Ferrite program that offers
`ReceiveValue<String, End>`, if we do not have any string value to send to it?

## `apply_channel`

```rust
fn apply_channel<A: Protocol, B: Protocol>(
  f: Session<ReceiveChannel<A, B>>,
  a: Session<A>,
) -> Session<B>
```

While we cannot execute Ferrite programs offering protocols other than `End`, we can
_link_ that Ferrite program with another program so that as a whole, the resulting Ferrite
would offer the protocol `End`. We have seen one such example of linking `hello_provider`
with `hello_client` using `apply_channel` in the
[previous chapter](../01-getting-started/03-communication.md).

In general, if we have a provider that offers the protocol `A`, and a client that offers
the protocol `ReceiveChannel<A, B>`, we can call `apply_channel(client, provider)` to
get a new program that would spawn the provider, and forwards the offered channel to
the client.


## `include_session`

When we have a Ferrite program that offers protocols like `ReceiveValue<i32, End>`, we could also
write another program that _include_ the first program directly using `include_session`
and interacts with it.
This allows us to write new Ferrite programs that offer `End` if we knows how to interact with the
original program.

As an example, consider a `show_number` program that receives an integer and then prints out the
value it receives:

```rust
{{#include ../../code/src/hello_4.rs:show_number}}
```

We cannot run `show_number` directly, because it expects an integer to be sent to it. But we can
now write a `main` program that includes `show_number` as follows:

```rust
{{#include ../../code/src/hello_4.rs:main}}
```

We call `include_session` by passing it the original `show_number` program that we want to include.
We also pass it a continuation closure, which gives us a channel variable `chan` that binds to
the channel offered by `show_number`. We then sends the number `42` to `chan`, wait for it to
terminate, and then terminate `main`.

Because the protocol offered by `main` is `End`, we can now run the full program using `run_session`.


## `run_session_with_result`


```rust
async fn run_session_with_result<T: Send + 'static>(
  session: Session<SendValue<T, End>>
) -> T
```

Although in general Ferrite can only run programs offering `End`, there is another special case
that Ferrite knows how to handle, which is programs that offer protocol in the form `SendValue<T, A>`.
In this case, Ferrite knows to receive the value sent by the program, then wait for the program to
terminate before returning the received value to the caller.

It is often more practical to use `run_session_with_result` instead of `run_session`, because we may
want to write Rust programs that spawn Ferrite processes, then wait for some result to return.

We have seen an example use of `run_session_with_result` with the
[hello provider](../../book/01-getting-started/02-hello-world.html), which we get back the
result "Hello World!" string sent by the Ferrite program.
