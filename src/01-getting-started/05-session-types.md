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
programming.

From the theoretical perspective,
session types are related to
[linear logic](https://plato.stanford.edu/entries/logic-linear/)
through the
[Curry–Howard correspondence](https://en.wikipedia.org/wiki/Curry%E2%80%93Howard_correspondence).
This is in contrast with function languages based on lambda calculus, such as Rust,
Haskell, and OCaml, which have correspondence with
[natural deduction](https://en.wikipedia.org/wiki/Natural_deduction).
Because of this, programming in session types may look unfamiliar to
typical functional programming.

Throughout this book, we will learn about various concepts of session types
from the perspective of a Rust programmer. While this may not be a comprehensive
overview to session types itself, we aim to cover enough of the basics
so that you can explore deeper into session types through other resources.

## Intuitionistic vs Classical Linear Logic

Since session types cover a family of programming languages, readers with
some familiarity with session types may notice that Ferrite's implementation
is quite different from some other session types implementations in
[the](https://github.com/Munksgaard/session-types)
[wild](https://github.com/wenkokke/sesh).

As a comparison, Ferrite's implementation of session types is based on
_intuitionistic_ linear logic. This is in contrast with the _classical_
linear logic formulation that some other session types implementations
are based on.

From the programmers' perspective, there are some key differences when
using the intuitionistic formulation of session types in Ferrite.
For example, you may notice that there is a lack of _dualization_ in
Ferrite. This means that the session type signature remain the
same from the perspective of both the provider and the client.

As we have seen in the [previous chapter](./03-communication.md),
while a provider sends a string value through a channel with
the session type `SendValue < String, End >`, the client _receives_
a value from a channel of the same session type. In comparison,
in a classical formulation, the session type of the same
channel for the client would instead have the session type
`ReceiveValue < String, End >`.

We leave it to informed readers to decide whether the intuitionistic formulation
of Ferrite provides better intuition for programming in session types.
For the purpose of this book, it is only important for us to clarify
the distinctions, so that readers are better informed when comparing
Ferrite with other session types implementations.

## Learn More

If you are interested to learn more about session types, there are
a wealth of type theoretic resources available. To get started,
we recommend watching the OPLSS lecture
[Session-Typed Concurrent Programming](https://www.youtube.com/watch?v=xYhGY0Lq8cw&list=PL0DsGHMPLUWWyBBTpnTBKsYxdkbIoy1q8)
given by Stephanie Balzer, our co-author for Ferrite.
The research group [ABCD](http://groups.inf.ed.ac.uk/abcd/index.html)
also publishes a comprehensive list of resources related to
session types, including a list of
[session types implementations](http://groups.inf.ed.ac.uk/abcd/session-implementations.html).
