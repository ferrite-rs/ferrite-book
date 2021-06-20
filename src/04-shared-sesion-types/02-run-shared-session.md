# Run Shared Session

```rust, noplaypen
fn run_shared_session<S>(
  session: SharedSession<S>
) -> SharedChannel<S>
```

- Just like `PartialSession`, `SharedSession` is an unevaluated shared
  session type program.
- `run_shared_session` evaluates a `SharedSession` and returns `SharedChannel`,
  a cloneable handle to communicate with the spawned shared process.

## Example

```rust, noplaypen
{{#include ../../code/src/constructs/shared.rs:run}}
```
