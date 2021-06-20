use ferrite_session::{
  either::*,
  prelude::*,
};

#[tokio::main]
async fn main()
{
  {
    // ANCHOR: offer_left
    let p: Session<
      InternalChoice<Either<ReceiveValue<String, End>, SendValue<u64, End>>>,
    > = offer_case!(
      Left,
      receive_value(move |name| {
        println!("Hello, {}!", name);
        terminate()
      })
    );
    // ANCHOR_END: offer_left
  }
  {
    // ANCHOR: offer_right
    let p: Session<
      InternalChoice<Either<ReceiveValue<String, End>, SendValue<u64, End>>>,
    > = offer_case!(Right, send_value(42, terminate()));
    // ANCHOR_END: offer_right
  }
  {
    // ANCHOR: case
    let p: Session<
      ReceiveChannel<
        InternalChoice<Either<ReceiveValue<String, End>, SendValue<u64, End>>>,
        End,
      >,
    > = receive_channel(move |c| {
      case! { c;
        Left =>
          send_value_to(c,
            "Alice".to_string(),
            wait(c, terminate())),
        Right =>
          receive_value_from(c, move |x| {
            println!("{}", x);
            wait(c, terminate())
          })
      }
    });
    // ANCHOR_END: case
  }
}
