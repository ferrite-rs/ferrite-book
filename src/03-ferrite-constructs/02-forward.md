# Forward

```rust, noplaypen
fn forward<N, C, A>(n: N) -> PartialSession<C, A>
```

- `C` is in the form `HList![…, N: A, …]` with the remaining elements
  being `Empty`.

Example:

```rust, noplaypen
{{#include ../../code/src/constructs/forward.rs:forward}}
```
