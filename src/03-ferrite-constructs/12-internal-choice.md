# Binary Internal Choice

## Provider Rule: Offer Left

```rust, noplaypen
macro offer_case! <C, A, B> (
  Left,
  $cont: PartialSession<C, A>
) ->
  PartialSession<C, InternalChoice<Either<A, B>>>
```

### Example

```rust, noplaypen
{{#include ../../code/src/constructs/internal_choice.rs:offer_left}}
```

## Provider Rule: Offer Right

```rust, noplaypen
macro offer_case! <C, A, B> (
  Right,
  $cont: PartialSession<C, B>
) ->
  PartialSession<C, InternalChoice<Either<A, B>>>
```

### Example

```rust, noplaypen
{{#include ../../code/src/constructs/internal_choice.rs:offer_right}}
```

## Client Rule: Case

```rust, noplaypen
macro case! <N, C1, C2, C3, A1, A2, B> {
  $n: N;
  Left => $cont1: PartialSession<C2, B>,
  Right => $cont1: PartialSession<C3, B>,
} ->
  PartialSession<C1, B>
```

- `C1` is in the form `HList[…, N: InternalChoice<Either<A1, A2>>, …]`.
- `C2` is in the form `HList[…, N: A1, …]`, with the remaining elements being the same as in `C1`.
- `C3` is in the form `HList[…, N: A2, …]`, with the remaining elements being the same as in `C1`.

### Example

```rust, noplaypen
{{#include ../../code/src/constructs/internal_choice.rs:case}}
```
