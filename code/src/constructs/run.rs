use std::time::Duration;

use ferrite_session::prelude::*;
use tokio::time::sleep;

#[tokio::main]
async fn main()
{
  {
    // ANCHOR: run
    let p: Session<End> = step(async move {
      sleep(Duration::from_secs(3)).await;
      println!("Hello World!");
      terminate()
    });

    run_session(p).await;
    // ANCHOR_END: run
  }
  {
    // ANCHOR: run_result
    let p: Session<SendValue<String, End>> = step(async move {
      sleep(Duration::from_secs(3)).await;
      send_value("Hello World!".to_string(), terminate())
    });

    let res = run_session_with_result(p).await;
    println!("{}", res);
    // ANCHOR_END: run_result
  }
}
