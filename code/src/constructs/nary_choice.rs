use ferrite_session::prelude::*;

// ANCHOR: either
define_choice! { Either<A, B>;
  Left: A,
  Right: B,
}
// ANCHOR_END: either

// ANCHOR: my_choices
define_choice! { MyChoices;
  Foo: ReceiveValue<String, End>,
  Bar: SendValue<u64, End>,
  Baz: End,
}
// ANCHOR_END: my_choices
