# Client for Shared Channels

## Acquire Shared Session

```rust, noplaypen
fn acquire_shared_session<N, C1, C2, A1, A2, B>(
  shared: SharedChannel<LinearToShared<A1>>,
  cont: impl FnOnce(N) -> PartialSession<C2, B>,
) -> PartialSession<C1, B>
```

### Example

Consider an equi-recursive definition

```rust, noplaypen
S = LinearToShared<SendValue<u64, SharedToLinear<S>>>
```

- `A1` = `SendValue<u64, Release>`
- `C1` = `HList![…]`
- `N` is the length of `C1`.
- ```rust, noplaypen
  C2 = HList![…,
    N: SendValue<u64,
        LinearToShared<
          SharedToLinear<
            SendValue<u64, Release>
          >>]
  ```

## Release Shared Session

```rust, noplaypen
fn release_shared_session<N, C1, C2, A, B>(
  n: N,
  cont: PartialSession<C2, B>,
) -> PartialSession<C1, B>
```

### Example

Consider an equi-recursive definition

```rust, noplaypen
S = LinearToShared<SendValue<u64, SharedToLinear<S>>>
```

- `A1` = `SendValue<u64, Release>`
- ```rust, noplaypen
  C1 = HList![…,
    N: LinearToShared<
        SharedToLinear<
          SendValue<u64, Release>
        >>,
    …]
  ```
- `C2` = `HList![…, N: Empty, …]`


## Example Client

```rust, noplaypen
{{#include ../../code/src/constructs/shared.rs:client}}
```
