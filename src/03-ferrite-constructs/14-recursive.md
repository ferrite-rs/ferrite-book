# Recursive Session Types

## Provider Rule: Fix Session

```rust, noplaypen
fn fix_session<C, A1, A2>(
  cont: PartialSession<C, A2>
) -> PartialSession<C, Rec<A1>>
```

- For a protocol `A = Rec<A1>`, `A1` contains recursion points back to `A` marked with `Z`.
- `A2` is `A1` with `Z` replaced by `Rec<A1>`.

### Example

```rust, noplaypen
{{#include ../../code/src/constructs/rec.rs:provider}}
```

For a equi-recursive definition `A = SendValue<u64, A>`:

- `A1` = `SendValue<u64, Z>`
- `A2` = `SendValue<u64, Rec<SendValue<u64, Z>>>`

## Client Rule: Unfix Session

```rust, noplaypen
fn unfix_session<N, C1, C2, A1, A2, B>(
  n: N,
  cont: PartialSession<C2, B>,
) -> PartialSession<C1, B>
```

- `C1` is in the form `HList[…, N: Rec<A1>, …]`
- `C2` is in the form `HList[…, N: A2, …]`, with the remaining elements
  being the same as in `C1`.

### Example

```rust, noplaypen
{{#include ../../code/src/constructs/rec.rs:client}}
```
