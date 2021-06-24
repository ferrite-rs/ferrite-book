# Shared Provider

## Accept Shared Session

```rust, noplaypen
fn accept_shared_session<A1, A2>(
  cont: Session<A2>
) -> SharedSession<LinearToShared<A1>>
```

- For a shared protocol `S = LinearToShared<A1>`, `A1` contains recursion points back to `SharedToLinear<S>` marked with `Release`.
- `A2` is `A1` with `Release` replaced by
  `SharedToLinear<LinearToShared<A1>>`
- Equi-synchronizing constraint applies, i.e. must use Release instead of End
- Returns a `SharedSession` instead of `PartialSession`

### Example

Consider an equi-recursive definition

```rust, noplaypen
S = LinearToShared<SendValue<u64, SharedToLinear<S>>>
```

- `A1` = `SendValue<u64, Release>`
- `A2` =
  ```rust, noplaypen
  SendValue<u64,
    LinearToShared<
      SharedToLinear<
        SendValue<u64, Release>>>
  ```


## Detach Shared Session

```rust, noplaypen
fn detach_shared_session<A>(
  cont: SharedSession<LinearToShared<A>>
) -> Session<SharedToLinear<A>>
```

- Continuation is a `SharedSession` instead of `PartialSession`
- Returns a `Session` with empty linear context.

## Example

```rust, noplaypen
{{#include ../../code/src/constructs/shared.rs:provider}}
```

## Shared Forward

```rust, noplaypen
fn shared_forward<A, C>(
  channel: SharedChannel<LinearToShared<A>>
) -> PartialSession<C, SharedToLinear<A>>
```

- Forward all subsequent shared acquires to another shared process connected to the given shared channel
