use ferrite_session::prelude::*;

#[tokio::main]
async fn main()
{
  {
    // ANCHOR: provider
    let p: Session<SendValue<String, End>> =
      send_value("Hello World!".to_string(), terminate());
    // ANCHOR_END: provider
  }
  {
    // ANCHOR: client
    let p: Session<ReceiveChannel<SendValue<String, End>, End>> =
      receive_channel(move |c| {
        receive_value_from(c, move |name| {
          println!("Hello, {}!", name);
          wait(c, terminate())
        })
      });
    // ANCHOR_END: client
  }
}
