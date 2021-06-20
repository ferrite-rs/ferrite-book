use ferrite_session::prelude::*;

#[tokio::main]
async fn main()
{
  {
    // ANCHOR: provider
    let greeter: Session<ReceiveValue<String, End>> =
      receive_value(move |name| {
        println!("Hello, {}!", name);
        terminate()
      });
    // ANCHOR_END: provider
  }
  {
    // ANCHOR: client
    let p: Session<ReceiveChannel<ReceiveValue<u64, End>, End>> =
      receive_channel(move |c| send_value_to(c, 42, wait(c, terminate())));
    // ANCHOR_END: client
  }
}
