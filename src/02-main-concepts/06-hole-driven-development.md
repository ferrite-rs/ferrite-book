# Hole Driven Development

Just like learning any new programming language, it may be overwhelming to
write your first session type program in Ferrite. To help you unstuck
when encountering compile errors, we encourage the use of _typed holes_
to discover what is needed to complete your program.

The use of typed holes are common in languages like Haskell, Agda, and Idris.
In Rust, there isn't officially support for using typed holes, but we can
somewhat emulate the feature using techniques described in this chapter.

## Todo Placeholder

Suppose we are a total beginner again, and we want to write our first
[hello world provider](../01-getting-started/02-hello-world.md).
We start by defining the session type we want, which is
`SendValue<String, End>`, and we can then write down the outline
for our Ferrite program as follows:

```rust, noplaypen
let hello_provider: Session<SendValue<String, End>> = todo!();
```

The [`todo!`](https://doc.rust-lang.org/std/macro.todo.html) macro allows
us to put a placeholder in unfinished Rust code so that we can try and
compile the code and see if there is any type error. By writing our code
step by step and filling the blank with `todo!()`, we can narrow down
the potential places where our code is incorrect.

At this stage, we should be able to compile our program with no error.
This shows that the protocol that we have defined, `SendValue<String, End>`,
is a valid session type. If we have gotten a compile error otherwise,
it could have been caused by us trying to write an invalid protocol
like `SendValue<String, String>`.

When using `todo!()`, the Rust compiler might emit warnings about
unreacheable code. Since we are still writing our program, we can
add the `#![allow(unreachable_code)]` pragma to disable the warning
temporarily until we finish writing the code.

## Unit Placeholder

Now that we have defined the outline, we can move to the next step
and try to write the first step. By looking at the first part
`SendValue<String, ...>`, we know that we have to send a string
value. But we might not know what should be done after sending
the value, so we could write something like follows:

```rust, noplaypen
let hello_provider: Session<SendValue<String, End>> =
  send_value("Hello World!".to_string(), todo!());
```

We can try to compile our code again, and Rust will accept the code
we have written. However the use of `todo!()` does not tell us
how we should continue our program. In languages like Haskell,
we could have used `_` instead of `todo!()`, and GHC would tell
us what should be the type of the expression in the hole. In Rust,
we would instead use the unit type `()` to deliberately cause
a compile error:

```rust, noplaypen
let hello_provider: Session<SendValue<String, End>> =
  send_value("Hello World!".to_string(), ());
```

Now if we compile our code, we would get a compile error from Rust
that looks like follows:

```
error[E0308]: mismatched types
 |
 |       send_value("Hello World!".to_string(), ());
 |                                              ^^ expected struct `PartialSession`, found `()`
 |
 = note: expected struct `PartialSession<(), End>`
         found unit type `()`

```

With this compile error, we can know that we are supposed to fill in the hole
with Rust expression that has the type `PartialSession<(), End>`, or `Session<End>`.

Sometimes we may also intuitively think of a type that should be in a hole.
In such case, we can also use the `todo!() as T` pattern to verify if our intuition
is correct. So we can for example write:

```rust, noplaypen
let hello_provider: Session<SendValue<String, End>> =
  send_value("Hello World!".to_string(), todo!() as Session<End>);
```

And our code will compile successfully. If we were to annotate it with an
invalid type, such as `todo()! as Session<ReceiveValue<String, End>>` again,
Rust will also return a compile error.

Now that we know the continuation needs to have the type `Session<End>`, we
can then fill in the blank with `terminate()` and complete our program.

## Underscores

Now that we have finished `hello_provider`, we can try implementing
[`hello_client`](../01-getting-started/03-communication.md) once again.
Supposed we know that the first expression to match `ReceiveChannel<...>`
is `receive_channel`, but we don't know what would be in the linear context,
we can write our program as follows:

```rust, noplaypen
let hello_client: Session<ReceiveChannel<SendValue<String, End>, End>> =
  receive_channel(move |a| {
    todo!() as PartialSession<HList![_], End>
  });
```

The code above will still compile successfully with Rust, so we know that
in our continuation, we have one element in the linear context as
denoted by `HList![_]`, but we don't yet know what it is.
Rust allows us to use `_` as a wildcard in places where it can infer the
type, such as the case when we use it in `todo!() as ...`. Using this
technique, we can also slowly narrow down our type definition and not
get overwhelmed by an overly long error message.

Now we can try and fill in the `_` with a bogus protocol like `End`
and see what happens:

```rust, noplaypen
let hello_client: Session<ReceiveChannel<SendValue<String, End>, End>> =
  receive_channel(move |a| {
    todo!() as PartialSession<HList![End], End>
  });
```

Now if we try to compile our program again, we would get a compile error
that looks like follows:

```
error[E0271]: type mismatch resolving `<() as AppendContext<(SendValue<String, End>, ())>>::Appended == (End, ())`
 |
 |       receive_channel(move |a| {
 |       ^^^^^^^^^^^^^^^ expected struct `SendValue`, found struct `End`
 |
 = note: expected type `(SendValue<String, End>, ())`
           found tuple `(End, ())`
```

The key information is in the last two lines, which states that there is a type mismatch
between the expected type `(SendValue<String, End>, ())` and the actual type `(End, ())`.
Recall from [previous chapter](./02-linear-context.md) that this is the desugared versions
of the expected list `HList![SendValue<String, End>]` and the actual list `HList![End]`.
Using this information, we are able to fill in the correct type for our hole:

```rust, noplaypen
let hello_client: Session<ReceiveChannel<SendValue<String, End>, End>> =
  receive_channel(move |a| {
    todo!() as PartialSession<HList![SendValue<String, End>], End>
  });
```

When writing complex session type programs, we can also use `_` in places
where we do not care about the actual type. For example, we may be interested
to know whether the there is one channel in the linear context in the form
`SendValue<String, ...>`. In that can we can also write our code as follows:

```rust, noplaypen
let hello_client: Session<ReceiveChannel<SendValue<String, End>, End>> =
  receive_channel(move |a| {
    todo!() as PartialSession<HList![SendValue<String, _>], _>
  });
```

Now that we know the first session type in the linear context is
in the form `SendValue<String, ...>`, we can attempt to write the next
part of our program. However suppose that we forgot that the behavior
of protocols is inverted on the client side, we may try to write
a program that sends a value to channel `a` instead of receiving from it:

```rust, noplaypen
let hello_client: Session<ReceiveChannel<SendValue<String, End>, End>> =
  receive_channel(move |a| {
    send_value_to(a, "Hello World!".to_string(), todo!())
  });
```

If we try to compile the code, we would get an error such as follows,
which is the same error as [previously described](./04-channel-selectors.md):

```
error[E0277]: the trait bound `Z: ContextLens<(SendValue<String, End>, ()), ReceiveValue<String, _>, _>` is not satisfied
 |
 |         send_value_to(a, "Hello World!".to_string(), todo!())
 |         ^^^^^^^^^^^^^ the trait `ContextLens<(SendValue<String, End>, ()), ReceiveValue<String, _>, _>` is not implemented for `Z`
 |
```

With the help of `todo!()`, we are able to spot the type errors in our program before finish
writing the whole program. It saves us the effort of figuring out what to fill in
the blank inside `send_value_to(a, "Hello World!".to_string(), ...)`, as our program
is incorrect anyway.

## Comment

Sometimes we may have written too much code, and by the time we try to compile our
program, we get a big list of compile errors that are difficult to decipher.
In such case, it may be helpful to comment out part of our Ferrite program
and replace them with `todo!()`, to find out the initial place where the
error occured.

For example, suppose we finished writing the wrong implementation of `hello_client`,
we can debug our code by commenting out the code to become something like:

```rust, noplaypen
let hello_client: Session<ReceiveChannel<SendValue<String, End>, End>> =
  receive_channel(move |a| {
    todo!()
    // send_value_to(a, "Hello World!".to_string(),
    //   wait(a, terminate()))
  });
```

If we build our program at this step, the compilation will be successful again.
We can then move forward to the next step of our program while still
commenting out the remaining part:

```rust, noplaypen
let hello_client: Session<ReceiveChannel<SendValue<String, End>, End>> =
  receive_channel(move |a| {
    send_value_to(a, "Hello World!".to_string(),
      todo!())
    //   wait(a, terminate()))
  });
```

At this step, we would then get a compile error. We can then know that
the cause of the error is from our use of `send_value_to`, and is
unaffected by the remaining expression `wait(a, terminate())`
because they have been commented out and replaced with `todo!()`.

## Rust IDE

When developing Ferrite programs, it may be helpful to use Rust IDEs
such as [Rust Analayzer](https://rust-analyzer.github.io/). The IDE
would show the hints on the types of the variables and functions.
Using and IDE, we can for example easily see that the type of
channel variables, such as `a` in `hello_client` has the type `Z`.
This can be useful especially for beginners, who may be unfamiliar
with the various types defined in Ferrite.
