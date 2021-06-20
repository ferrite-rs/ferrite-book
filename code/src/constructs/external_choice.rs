use ferrite_session::{
  either::*,
  prelude::*,
};

#[tokio::main]
async fn main()
{
  {
    // ANCHOR: provider
    use ferrite_session::{
      either::*,
      prelude::*,
    };

    let p: Session<
      ExternalChoice<Either<ReceiveValue<String, End>, SendValue<u64, End>>>,
    > = offer_choice! {
      Left =>
        receive_value(move |name| {
          println!("Hello, {}!", name);
          terminate()
        })
      Right =>
        send_value(42, terminate())
    };
    // ANCHOR_END: provider
  }
  {
    // ANCHOR: choose_left
    let p: Session<
      ReceiveChannel<
        ExternalChoice<Either<ReceiveValue<String, End>, SendValue<u64, End>>>,
        End,
      >,
    > = receive_channel(move |c| {
      choose!(
        c,
        Left,
        send_value_to(c, "Hello World!".to_string(), wait(c, terminate()))
      )
    });
    // ANCHOR_END: choose_left
  }
  {
    // ANCHOR: choose_right
    let p: Session<
      ReceiveChannel<
        ExternalChoice<Either<ReceiveValue<String, End>, SendValue<u64, End>>>,
        End,
      >,
    > = receive_channel(move |c| {
      choose!(
        c,
        Right,
        receive_value_from(c, move |x| {
          println!("{}", x);
          wait(c, terminate())
        })
      )
    });
    // ANCHOR_END: choose_right
  }
}
