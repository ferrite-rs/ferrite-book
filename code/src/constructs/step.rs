use ferrite_session::prelude::*;

#[tokio::main]
async fn main()
{
  // ANCHOR: step
  use std::time::Duration;

  use tokio::time::sleep;

  let p: Session<End> = step(async move {
    sleep(Duration::from_secs(1)).await;
    terminate()
  });
  // ANCHOR_END: step
}
