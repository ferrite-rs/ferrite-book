# Channel Selectors

In the code for [`hello_client`](../01-getting-started/03-communication.md), we have seen
that the `receive_channel` construct introduces a new channel variable to our continuation,
which we bind to the variable `a`. The channel variable `a` is then used in both
`receive_value_from(a, ...)` and `wait(a, ...)` to access the given channel in the linear context.
In this chapter, we will peek a bit into the mechanics of how the channel variables
provide access to channels in the linear context.

## Context Lenses

Recall from the [previous chapter](./02-linear-context.md) that a linear context
has the form `HList![A0, A1, ...]`, with each element implementing `Protocol`.
During the execution of a Ferrite program, the session type of the channels
in the linear context will be updated in each step, until the final linear
context has the form `HList![Empty, Empty, ...]` to be safely terminated.

When some Ferrite constructs such as `receive_value_from` are called, a channel
variable needs to be given so that the construct can know which channel
in the linear context it should access. Since the linear context is represented
as a type level list, the channels are in fact not named in the linear context.
Instead, channel variables such as `a` in `hello_client` actually have types
that implement _context lenses_ to access channels in the linear context
by _position_.

Conceptually, a type `N` that implements the `ContextLens` trait provides
access to a particular element in the type level list, and update it
to a new type. For example in `hello_client`, the type of `a` implements
`ContextLens` for accessing the linear contexts
`HList![SendValue<String, End>]`, and updates it to `HList[End]`, then
finally updates it to `HList![Empty]`.

## Type Level Natural Numbers

To define types that implement the `ContextLens` trait, Ferrite uses
_type level natural numbers_ to implement the access to channels
in the type level list by the corresponding position. A type level
natural number starts with the type `Z`, which corresponds to the
number 0. Following that, we have the type `S<Z>`, which means
the _successor_ of `Z`, to correspond to the number 1.
Similarly, we have `S<S<Z>>` corresponding to the successor of 1,
which is 2, and so on. So we have the types
`Z`, `S<Z>`, `S<S<Z>>`, `S<S<S<Z>>>`, ... corresponding to
the number sequence 0, 1, 2, 3, ...

For each of the type level natural numbers, Ferrite implements
the `ContextLens` trait for accessing channels at the corresponding
zero-indexed position in the linear context. So for example:

  - The type `Z` can update any linear context in the form `HList![A0, ...]` to
    `HList![B, ...]`, with the first element `A0` replaced with `B` and the
    remaining elements unchanged.

  - The type `S<Z>` can update any linear context in the form `HList![A0, A1, ...]` to
    `HList![A0, B, ...]`, with the second element `A1` replaced with `B` and
    the remaining elements unchanged.

  - The type `S<S<Z>>` can update any linear context in the form `HList![A0, A1, A2, ...]` to
    `HList![A0, A1, B, ...]`, with the third element `A3` replaced with `B` and
    the remaining elements unchanged.

As demonstration, we can deduce that the channel variable `a` in `hello_client` has
the type `Z`, because there is only one channel in the linear context and it is
the one that we are interested in. `Z` implements the `ContextLens` for updating
the first element in any linear context. So `receive_channel_from` can use `Z`
to update the linear context from `HList![ReceiveValue<String, End>]` to `HList![End]`,
and it can also use `Z` to update the linear context from `HList![End]` to `HList![Empty]`.

## The `ContextLens` Trait

The full definition of the `ContextLens` trait is in the form `ContextLens<C1, A, B, Target=C2>`.
It means that a type `N` that implements `ContextLens<C1, A, B, Target=C2>` provides
the operation to access a channel of session type `A` that can be found in `C1`,
update it to `B`, and result in the new context `C2`. This can be elaborated with
the implementations by the natural numbers:

  - `Z: ContextLens<HList![A0, ...], A0, B, Target=HList![B, ...]>`

  - `S<Z>: ContextLens<HList![A0, A1, ...], A0, B, Target=HList![A0, B, ...]>`

  - `S<S<Z>>: ContextLens<HList![A0, A1, A2,...], A0, B, Target=HList![A0, A1, B, ...]>`

Following the above template, we can know that in the case of `hello_client`, the type `Z`
does implements the required `ContextLens` traits:

  - ```
    Z: ContextList<
          HList![ReceiveValue<String, End>],
          ReceiveValue<String, End>,
          End,
          Target=HList![End]>
    ```

  - ```
    Z: ContextList<
          HList![End],
          End,
          Empty,
          Target=HList![Empty]>
    ```

## Writing Partial Ferrite Programs

Now that we know how context lenses are actually implemented, it would in fact
be possible for us to write partial Ferrite programs of type `PartialSession`,
by hard coding the specific context lenses to access the channels in the linear context.
For example, we could have written a partial version of `hello_client` as follows:


```rust
{{#include ../../code/src/hello_2.rs:hello_client_partial}}
```

While writing such program is possible, the code is not ergonomic and is generally
not recommended. This is because we are hardcoding the position of the channels
in the linear context, instead of using meaningful identifiers. This would also
reduce readability and makes it more difficult to add or remove channels to
the program.

This style of programming using the position is similar to programming using
[De Bruijn index](https://en.wikipedia.org/wiki/De_Bruijn_index). While this
is useful for internal implementation, we as programmers tend to prefer
using named variables to improve readability and better structure our code.


## Type Errors Using Context Lenses

When we write Ferrite programs that contain invalid access to channels in the linear context,
we may get compile errors showing that types like `Z` do not implement a particular context lens.
As an example, consider the following incorrect version of `hello_client` that tries to
send a value to `a` instead of receiving from it:

```rust
let hello_client_incorrect
  : Session<ReceiveChannel<SendValue<String, End>, End>>
  = receive_channel(|a| {
      send_value_to(a, "Hello World!".to_string(),
        wait(a, terminate()))
    });
```

If we try to compile the program, we would get an error message similar to the following:

```
error[E0277]: the trait bound `Z: ContextLens<(SendValue<String, End>, ()), ReceiveValue<String, _>, _>` is not satisfied
   |
20 |     send_value_to(a, "Hello World!".to_string(),
   |     ^^^^^^^^^^^^^ the trait `ContextLens<(SendValue<String, End>, ()), ReceiveValue<String, _>, _>` is not implemented for `Z`
   |
   |
49 |   N: ContextLens<C, ReceiveValue<T, B>, B>,
   |      ------------------------------------- send_value_to`
   |
   = help: the following implementations were found:
             <Z as ContextLens<(A1, C), A1, A2>>
```

The key meaning for the error message above is that the type `Z` does not implement
`ContextLens<HList![SendValue<String, End>], ReceiveValue<String, End>, End>`.
This is because although `Z` implements access to the first element of any
non-empty linear context, the first element in the linear context of `hello_client`
has the protocol `SendValue<String, End>`, but the call to `send_value_to(a, ...)`
requires the first element of the linear context to have the protocol
`ReceiveValue<String, End>`.

Error messages such as above may be difficult to understand for readers who are
new to Ferrite. But hopefully with some exercise, we should be able to get familiar
with the error message and understand the meaning behind them.
