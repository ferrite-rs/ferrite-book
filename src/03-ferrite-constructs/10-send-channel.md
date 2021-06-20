# Send Channel

## Provider Rule: Send Channel From

```rust, noplaypen
fn send_channel_from<N, C1, C2, A, B>(
  n: N,
  cont: PartialSession<C2, B>,
) -> PartialSession<C1, SendChannel<A, B>>
```

- `C1` is in the form `HList[…, N: A, …]`.
- `C2` is in the form `HList[…, N: Empty, …]`, with the remaining elements being same as in `C1`.

### Example

```rust, noplaypen
{{#include ../../code/src/constructs/send_channel.rs:provider}}
```

## Client Rule: Receive Channel From

```rust, noplaypen
fn receive_channel_from<C1, C2, N, M, A1, A2, B>(
  n: N,
  cont: impl FnOnce(M) -> PartialSession<C2, B>,
) -> PartialSession<C1, B>
```

- `M` is the length of `C1`.
- `C1` is in the form `HList[…, N: SendChannel<A1, A2>, …]`.
- `C2` is in the form `HList[…, N: A2, …, M: A1]`, with all elements
  in `C1` except slot `N` unchanged, and have `M` appended to the end.

### Example

```rust, noplaypen
{{#include ../../code/src/constructs/send_channel.rs:client}}
```
