# Session Types

This chapter provides some supplementary overview of session types,
the type theory that Ferrite is based on. In the discussion we
will talk about some concepts related to
[logic](https://en.wikipedia.org/wiki/Mathematical_logic) and
[type theory](https://en.wikipedia.org/wiki/Type_theory).
For readers who are unfamiliar with these topics, feel free
to skip this chapter and move on to the
[next chapter](./06-macros.md).

## Background on Session Types

So far we have been talking about session types without going into details
what it is. Broadly speaking, session types can refer to a branch of
type systems used to model structured communication-based
programming. Session types are also known as _behavioural types_, since
the types describe the behaviour of a program.

From the theoretical perspective, session types establish a
[correspondence](https://en.wikipedia.org/wiki/Curry%E2%80%93Howard_correspondence)
with [linear logic](https://plato.stanford.edu/entries/logic-linear/).
This makes session-typed languages slightly different from functional
languages such as Rust and Haskell, which have their roots in
[lambda calculus](https://plato.stanford.edu/entries/lambda-calculus/)
and
[intuitionistic logic](https://plato.stanford.edu/entries/logic-intuitionistic/).

Throughout this book, we will learn about various concepts of session types
from the perspective of a Rust programmer. While this may not be a comprehensive
overview of session types themselves, we aim to cover enough of the basics
so that you can explore more deeply into session types through other resources.

## Linear, Affine, and Structural Rules

One notable distiction between session types and Rust is that session typed
variables are _linear_, while Rust variables are _affine_ by default.
We will try to understand what that really means.

A common explanation of linear vs affine is that linear variables must be
used exactly once, while affine variables can be used at most once.
However that is not entirely accurate.

To be more precise, type theorists differentiate different kinds of
type systems based on which
[structural rules](https://en.wikipedia.org/wiki/Structural_rule)
are allowed in the type system:

  - In a normal type system like Haskell, variables are allowed to be used
    more than once (contraction). Variables are also allowed to be discarded
    (weakening).

  - In an affine type system like Rust, variables are _not_ allowed to be used
    more than once (no contraction), unless they specifically implement
    `Copy` or `Clone`.
    However variables are still allowed to be discarded (weakening).

  - In a linear type system like Ferrite, variables are _not_ allowed to be
    used more than once (no contraction), and variables are _not_ allowed to
    be discarded (no weakening).

As we have seen in the previous chapter, Ferrite programs like `hello_client`
interact with channel variables multiple times, with the session type of
the channel changing every time after interaction. Under linear typing rules,
we cannot make multiple copies of the channel, and we also cannot discard
the channel until it is explicitly terminated.

## Intuitionistic vs Classical Linear Logic

Since session types cover a family of programming languages, readers with
some familiarity with session types may notice that Ferrite's implementation
is quite different from some other session types implementations in the wild,
such as [session-types](https://github.com/Munksgaard/session-types)
and [sesh](https://github.com/wenkokke/sesh).

Ferrite's implementation of session types is based on
_intuitionistic_ linear logic. This is in contrast with the _classical_
linear logic formulation that is used by some other session types
implementations.

From the programmers' perspective, there are some key differences when
using the intuitionistic formulation of session types in Ferrite.
For example, you may notice that there is no need for _dualization_
in Ferrite. This means that the session type signature remain the
same from the perspective of both the provider and the client.

As illustrated in the [previous chapter](./03-communication.md),
while the provider `hello_provider` sends a string value through
a channel with the session type `SendValue < String, End >`,
the client `hello_client` _receives_ a value from a channel of the
same session type.

In comparison, with classical formulation, we would instead
have to explicitly dualize the session type. As a result,
the session type of the channel given to `hello_client` would
instead have the session type `ReceiveValue < String, End >`.

Both intuitionistic and classical formulation of session types
share the same foundation, viewed from different angles.
From the perspective of a type theorist, it may be more
convenient to formulate proofs with the help of dualization
in the classical formulation.

From the perspective of a programmer, it may be more intuitive to understand
session types in the intuitionistic formulation. Intuitionistic session types
have closer semantics with lambda calculus, and this makes it easier for
programmers to bridge the concepts with their existing knowledge in
functional programming.

We leave it to informed readers to decide whether the intuitionistic formulation
of Ferrite provides better intuition for programming in session types.
For the purpose of this book, it is only important for us to clarify
the distinctions, so that readers are better informed when comparing
Ferrite with other session types implementations.

## Learn More

If you are interested to learn more about session types, there are
a wealth of type theoretic resources available. To get started,
we recommend the following learning materials by our co-author:

  - Tutorial slides - [Session-Typed Concurrent Programming](http://www.cs.cmu.edu/~balzers/publications/session_typed_concurrent_programming.pdf)

  - OPLSS video lectures - [Session-Typed Concurrent Programming](https://www.youtube.com/watch?v=xYhGY0Lq8cw&list=PL0DsGHMPLUWWyBBTpnTBKsYxdkbIoy1q8)

Other than that, the research group [ABCD](http://groups.inf.ed.ac.uk/abcd/index.html)
also publishes a comprehensive list of resources related to
session types, including a list of
[session types implementations](http://groups.inf.ed.ac.uk/abcd/session-implementations.html).
