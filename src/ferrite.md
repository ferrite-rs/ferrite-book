# Writing Session-Typed Programs in Ferrite (Draft)

Welcome to Writing Session-Typed Programs in Ferrite!
This is an introductory guide on how to use
[Ferrite](https://github.com/maybevoid/ferrite),
an EDSL (embedded domain specific language),
for writing session-typed programs in Rust.

In this book you will learn about writing Ferrite programs
with _linear_ session types, which channel variables must be used
_exactly once_. This is in contrast with normal Rust programs with
_affine_ types, which variables can be used _at most once_.

You will learn how session types can be used to define
_communication protocols_ between different parts of a Rust program.
You will see how Ferrite can help elminate the boilerplates of managing
communication using plain Rust channels, and enforce protocol
compliance through linearity.

Finally you will also learn about _shared session types_, which
you can use to define services that can be safely shared among
multiple linear clients. You will learn how shared session types
can be used to implement common communication patterns
such as client-server communication.


## Work In Progress

Ferrite is currently still in development, with parts of the code subject
to change. This book is also a work in progress, with most of the content
yet to be written. Keep an eye on the
[ferrite](https://github.com/maybevoid/ferrite)
and [ferrite-book](https://github.com/maybevoid/ferrite-book) repositories
for any update.