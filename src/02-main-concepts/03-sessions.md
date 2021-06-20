# Sessions

We have seen that Ferrite programs have the Rust type `Session<A>`, where
the type `A` must implement the `Protocol` trait. A Ferrite program
of type `Session<A>` starts with an empty linear context, and offers
the protocol `A`. More generally, it is also possible to define _partial_
Ferrite programs with non-empty linear context. In such cases, these
programs would have the Rust type `PartialSession<C, A>`.

## Partial Sessions

A partial Ferrite program of type `PartialSession<C, A>` have a linear
context `C` which implements `Context`, and offers the session type `A`.
The linear context `C` may be non-empty, and if so the partial program
must fully consume the linear context before it can terminate.

In the special case when the linear context is empty, the partial program
is then equivalent to the full Ferrite program. In fact, the type
`Session<A>` is defined to be a type alias of `PartialSession<(), A>`,
or `PartialSession<HList![], A>`.

When we write our Ferrite program, the program is actually made of
many fragments of `PartialSession` that are composed together.
For example, the program fragment `wait(a, terminate())` in
`hello_client` has the Rust type `PartialSession<HList![End], End>`,
and the program fragment `receive_value_from(a, wait(a, terminate()))`
has the Rust type `PartialSession<HList![SendValue<String, End>], End>`.

Ferrite constructs such as `receive_value_from` implements the typing
rules of how the linear context and the offered protocol should be
updated before and after the given Ferrite expression. For example,
given that the channel variable `a` refers to the first element of
the linear context, the Rust expression `receive_value_from(a, cont)`
should have the type in the form `PartialSession<HList![SendValue<T, A>], B>`,
and the continuation expression `cont` should have the Rust type in the form
`PartialSession<HList![A], B>`. i.e. `receive_value_from` _updates_
the protocol of the first channel in the linear context from `SendValue<T, A>`
to `A` for any protocol `A`.

## Closed Ferrite Programs

Now that we have learned about `PartialSession`, a question that might arise is
why don't we just define `hello_client` to have the type
`PartialSession<HList![ReceiveValue<String, End>], End>` instead of
`Session<ReceiveChannel<ReceiveValue<String, End>, End>>`?
The reason is because a partial session has _free variables_ that
are not captured by the program. Consider the following invalid program:

```rust, noplaypen
let hello_client_incorrect
  : PartialSession<HList![ReceiveValue<String, End>], End>
  = receive_value_from(a, move |greeting| {
      println!("Received greetings from provider: {}", greeting);
      wait(a, terminate())
    });
```

The channel variable `a` is not defined anywhere, or in other words we
have a free variable `a`, hence the program would not compile.
We can think of defining partial sessions directly being equivalent
to defining Rust expressions containing free variables such as `x + 1`.
Without the surrounding code, we wouldn't able to know where `x` is from
and whether the expression is valid with a given `x`. To make such
expression to be valid, we would instead define a _closure_ that
captures the free variable `x`, forming the _closed_ expression
`|x| { x + 1 }`.

The same principle also applies to Ferrite programs.
In practice, we would write Ferrite programs having the `Session` type
instead of `PartialSession` so that there are no free channel variables
that escape the program.
