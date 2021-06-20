use ferrite_session::prelude::*;

#[tokio::main]
async fn main()
{
  // ANCHOR: forward
  let p: Session<
    ReceiveChannel<ReceiveValue<String, End>, ReceiveValue<String, End>>,
  > = receive_channel(move |c| forward(c));
  // ANCHOR_END: forward
}
