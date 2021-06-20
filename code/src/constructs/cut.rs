use ferrite_session::prelude::*;

#[tokio::main]
async fn main()
{
  {
    // ANCHOR: include
    let p1: Session<End> = terminate();
    let p2: Session<End> = include_session(p1, move |c| wait(c, terminate()));
    // ANCHOR_END: include
  }
  {
    // ANCHOR: apply_channel
    let p1: Session<ReceiveValue<String, End>> = todo!();

    let p2: Session<
      ReceiveChannel<ReceiveValue<String, End>, SendValue<u64, End>>,
    > = todo!();

    let p3: Session<SendValue<u64, End>> = apply_channel(p2, p1);
    // ANCHOR_END: apply_channel
  }
}
