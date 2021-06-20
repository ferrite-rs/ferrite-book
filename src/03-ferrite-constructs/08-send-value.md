# Send Value

## Provider Rule: Send Value

```rust, noplaypen
fn send_value<T, C, A>(
  val: T,
  cont: PartialSession<C, A>,
) ->
  PartialSession<C, SendValue<T, A>>
```

### Example

```rust, noplaypen
{{#include ../../code/src/constructs/send_value.rs:provider}}
```

## Client Rule: Receive Value From

```rust, noplaypen
fn receive_value_from<N, C1, C2, T, A, B>(
  n: N,
  cont: impl FnOnce(T) -> PartialSession<C2, B>,
) -> PartialSession<C1, B>
```

- `C1` is in the form `HList[…, N: SendValue<T, A>, …]`.
- `C2` is in the form `HList[…, N: A, …]`, with the remaining elements
  being the same as `C1`.

### Example

```rust, noplaypen
{{#include ../../code/src/constructs/send_value.rs:client}}
```
