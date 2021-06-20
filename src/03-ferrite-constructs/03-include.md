# Include Session

```rust, noplaypen
fn include_session<N, C1, C2, A, B>(
  session: Session<A>,
  cont: impl FnOnce(N) -> PartialSession<C2, B>,
) -> PartialSession<C1, B>
```

- `N` is the length of `C1`.
- `C1` is in the form `HList![0: A0, 1: A1, ...]`
- `C2` is in the form `HList![0: A0, 1: A1, ..., N: A]`,
  with `A` appended to the end of `C1` and the remaining elements unchanged.

### Example

```rust, noplaypen
{{#include ../../code/src/constructs/cut.rs:include}}
```
