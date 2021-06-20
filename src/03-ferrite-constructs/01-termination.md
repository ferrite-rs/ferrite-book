# Termination

## Provider Rule: Terminate

```rust, noplaypen
fn terminate<C>() -> PartialSession<C, End>
```

- `C` must be an empty linear context that implements `EmptyContext`. e.g:
  - `HList![]`
  - `HList![Empty]`
  - `HList![Empty, Empty, ...]`

Example:

```rust, noplaypen
{{#include ../../code/src/constructs/termination.rs:terminate}}
```

## Client Rule: Wait

```rust, noplaypen
fn wait<N, C1, C2, A>(
  n: N,
  cont: PartialSession<C2, A>
) -> PartialSession<C1, A>
```

- `C1` is in the form `HList[…, N: End, …]`.
- `C2` is in the form `HList[…, N: Empty, …]` with the remaining elements
  in `C1` unchanged.

Example:

```rust, noplaypen
{{#include ../../code/src/constructs/termination.rs:wait}}
```

## Client Rule: Wait All

```rust, noplaypen
macro wait_all! <C1, C2, B> (
  [$channels],
  $cont: PartialSession<C2, B>
) -> PartialSession<C1, B>
```

- Expands `wait_all!([c1, c2, …], cont)` to `wait(c1, wait(c2, … cont)`.

Example:

```rust, noplaypen
{{#include ../../code/src/constructs/termination.rs:wait_all}}
```
