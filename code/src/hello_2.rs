#![allow(dead_code)]

// ANCHOR: hello_2
use ferrite_session::prelude::*;

type Hello = SendValue<String, End>;

#[tokio::main]
async fn main()
{

// ANCHOR: hello_provider
let hello_provider: Session<Hello> =
  send_value("Hello World!".to_string(), terminate());
// ANCHOR_END: hello_provider

// ANCHOR: hello_client
let hello_client: Session<ReceiveChannel<Hello, End>> =
  receive_channel(|provider| {
    receive_value_from(provider, move |greeting| {
      println!("Received greetings from provider: {}", greeting);
      wait(provider, terminate())
    })
  });
// ANCHOR_END: hello_client

// ANCHOR: apply_channel
let main: Session<End> = apply_channel(hello_client, hello_provider);
// ANCHOR_END: apply_channel

// ANCHOR: run_session
run_session(main).await;
// ANCHOR_END: run_session

}
// ANCHOR_END: hello_2

#[test]
fn test_main()
{
  main();
}
