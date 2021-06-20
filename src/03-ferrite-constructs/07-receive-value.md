# Receive Value

## Provider Rule: Receive Value

```rust, noplaypen
fn receive_value<T, C, A>(
  cont: impl FnOnce(T) -> PartialSession<C, A>
) ->
  PartialSession<C, ReceiveValue<T, A>>
```

- `T: Send + 'static`, i.e. `T` must not contain borrowed reference and is safe
  to be shared across threads.

### Example

```rust, noplaypen
{{#include ../../code/src/constructs/receive_value.rs:provider}}
```

## Client Rule: Send Value To

```rust, noplaypen
fn send_value_to<N, C1, C2, A, B, T>(
  n: N,
  val: T,
  cont: PartialSession<C2, B>,
) -> PartialSession<C1, B>
```

- `C1` is in the form `HList[…, N: ReceiveValue<T, A>, …]`.
- `C2` is in the form `HList[…, N: A, …]`, with the remaining elements
  unchanged.

### Example:

```rust, noplaypen
{{#include ../../code/src/constructs/receive_value.rs:client}}
```
