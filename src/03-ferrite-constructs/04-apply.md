# Apply Channel

```rust, noplaypen
fn apply_channel<A, B>(
  f: Session<ReceiveChannel<A, B>>,
  a: Session<A>,
) -> Session<B>
```

Example:

```rust, noplaypen
{{#include ../../code/src/constructs/cut.rs:apply_channel}}
```
