use ferrite_session::prelude::*;

#[tokio::main]
async fn main()
{
  {
    // ANCHOR: provider
    let p: Session<ReceiveChannel<End, End>> =
      receive_channel(move |c| wait(c, terminate()));
    // ANCHOR_END: provider
  }
  {
    // ANCHOR: client
    let p1: Session<ReceiveValue<String, End>> = todo!();

    let p2: Session<ReceiveChannel<ReceiveValue<String, End>, End>> = todo!();

    let p3: Session<End> = include_session(p1, move |c1| {
      include_session(p2, move |c2| {
        send_channel_to(c2, c1, wait(c2, terminate()))
      })
    });
    // ANCHOR_END: client
  }
}
