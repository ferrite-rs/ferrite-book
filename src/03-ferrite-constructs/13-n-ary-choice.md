# N-ary Choice

```rust, noplaypen
macro define_choice! {
  $choice ;
  $label1: $protocol1,
  $label2: $protocol2,
  ...
}
```

- Define N branches of labeled choices
- Choices are in the form `{ L0: A0, L1: A1, ...  }` are desugard into
  `HList![A0, A1, ...]`.
- Labeled are defined as type aliases to _prisms_, e.g.

  ```rust, noplaypen
  type L0 = Z;
  type L1 = S<Z>;
  ...
  ```

## Example: Either

```rust, noplaypen
{{#include ../../code/src/constructs/nary_choice.rs:either}}
```

## Example

```rust, noplaypen
{{#include ../../code/src/constructs/nary_choice.rs:my_choices}}
```
