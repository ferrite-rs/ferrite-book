# Receive Channel

## Provider Rule: Receive Channel

```rust, noplaypen
fn receive_channel<N, C1, C2, A, B>(
  cont: impl FnOnce(N) -> PartialSession<C2, B>
) -> PartialSession<C1, ReceiveChannel<A, B>>
```

- `N` is the length of `C1`.
- `C1` is in the form `HList![0: A0, 1: A1, ...]`
- `C2` is in the form `HList![0: A0, 1: A1, ..., N: A]`,
  with `A` appended to the end of `C1` and the remaining elements unchanged.

### Example

```rust, noplaypen
{{#include ../../code/src/constructs/receive_channel.rs:provider}}
```

## Client Rule: Send Channel To

```rust, noplaypen
fn send_channel_to <N, M, C1, C2, A1, A2, B>(
  n: N, m: M,
  cont: PartialSession<C2, B>,
) -> PartialSession<C1, B>
```

- `C1` is in the form `HList[…, N: ReceiveChannel<A1, A2>, … M: A1, …]`
- `C2` is in the form `HList[…, N: A2, …, M: Empty, …]`, with the remaining elements being same as `C1`.
- `M` may come before `N`.

### Example

```rust, noplaypen
{{#include ../../code/src/constructs/receive_channel.rs:client}}
```
