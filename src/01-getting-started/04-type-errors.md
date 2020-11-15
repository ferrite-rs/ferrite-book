# Type Errors

```rust
let hello_client :
  Session <
    ReceiveChannel < Hello, End >
  > =
  receive_channel! ( provider => {
    terminate! ()
  });
```

```
error[E0277]: the trait bound `(SendValue<String, End>, ()): EmptyContext` is not satisfied
  |
  |  terminate! ()
  |  ^^^^^^^^^^^^^ the trait `EmptyContext` is not implemented for `(SendValue<String, End>, ())`
```

## Linear Context


(TODO)
