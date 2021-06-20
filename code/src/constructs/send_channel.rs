use ferrite_session::prelude::*;

#[tokio::main]
async fn main()
{
  {
    // ANCHOR: provider
    let p1: Session<SendValue<String, End>> = todo!();

    let p: Session<SendChannel<SendValue<String, End>, End>> =
      include_session(p1, move |c| send_channel_from(c, terminate()));
    // ANCHOR_END: provider
  }
  {
    // ANCHOR: client
    let p: Session<
      ReceiveChannel<SendChannel<ReceiveValue<String, End>, End>, End>,
    > = receive_channel(move |c1| {
      receive_channel_from(c1, move |c2| {
        send_value_to(
          c2,
          "Hello World!".to_string(),
          wait_all!([c1, c2], terminate()),
        )
      })
    });
    // ANCHOR_END: client
  }
}
