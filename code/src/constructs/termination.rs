use ferrite_session::prelude::*;

#[tokio::main]
async fn main()
{
  {
    // ANCHOR: terminate
    let p: Session<End> = terminate();
    // ANCHOR_END: terminate
  }

  {
    // ANCHOR: wait
    let p: Session<ReceiveChannel<End, End>> =
      receive_channel(move |c| wait(c, terminate()));
    // ANCHOR_END: wait
  }
  {
    // ANCHOR: wait_all
    let p: Session<ReceiveChannel<End, ReceiveChannel<End, End>>> =
      receive_channel(move |c1| {
        receive_channel(move |c2| wait_all!([c1, c2], terminate()))
      });
    // ANCHOR_END: wait_all
  }
}
