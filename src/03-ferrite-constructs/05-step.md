# Step

```rust, noplaypen
fn step<C, A>(
  cont: impl Future< Output = PartialSession<C, A>>
) -> PartialSession<C, A>
```

- Wraps an async block yielding a `PartialSession`.

Example:

```rust, noplaypen
{{#include ../../code/src/constructs/step.rs:step}}
```
