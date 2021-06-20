# Binary External Choice

## Provider Rule: Offer Choice

```rust, noplaypen
macro offer_choice! <C, A, B> {
  Left => $expr: PartialSession<C, A>,
  Right => $expr: PartialSession<C, B>,
} ->
  PartialSession<C, ExternalChoice<Either<A, B>>>
```

### Example

```rust, noplaypen
{{#include ../../code/src/constructs/external_choice.rs:provider}}
```

## Client Rule: Choose Left

```rust, noplaypen
macro choose! <N, C1, C2, A1, A2, B> (
  n: N, Left,
  cont: PartialSession<C2, B>
) ->  PartialSession<C1, B>
```

- `C1` is in the form `HList[…, N: ExternalChoice<Either<A1, A2>>, …]`
- `C2` is in the form `HList[…, N: A1, …]`, with the remaining elements
  being the same as in `C1`.


### Example

```rust, noplaypen
{{#include ../../code/src/constructs/external_choice.rs:choose_left}}
```

## Client Rule: Choose Right

```rust, noplaypen
macro choose! <N, C1, C2, A1, A2, B> (
  $n: N, Right,
  $cont: PartialSession<C2, B>
) ->  PartialSession<C1, B>
```

- `C1` is in the form `HList[…, N: ExternalChoice<Either<A1, A2>>, …]`
- `C2` is in the form `HList[…, N: A2, …]`, with the remaining elements
  being the same as in `C1`.


### Example

```rust, noplaypen
{{#include ../../code/src/constructs/external_choice.rs:choose_right}}
```
