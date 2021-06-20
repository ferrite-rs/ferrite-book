# Run Session

```rust, noplaypen
async fn run_session(session: Session<End>)
```

- Evaluates a `Session<End>` and blocks until the process terminates.

Example:

```rust, noplaypen
{{#include ../../code/src/constructs/run.rs:run}}
```

## Run Session With Result

```rust, noplaypen
async fn run_session_with_result<T>(
  session: Session<SendValue<T, End>>
) -> T
```

- Evaluates a `Session<SendValue<T, End>>` and blocks until the
  process terminates
- Returns the result sent by the process

Example:

```rust, noplaypen
{{#include ../../code/src/constructs/run.rs:run_result}}
```
