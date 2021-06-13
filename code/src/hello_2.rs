#![allow(dead_code)]
#![allow(unused_variables)]

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
    receive_channel(|a| {
      receive_value_from(a, move |greeting| {
        println!("Received greetings from provider: {}", greeting);
        wait(a, terminate())
      })
    });
  // ANCHOR_END: hello_client

  // ANCHOR: hello_client_partial
  let hello_client_partial: PartialSession<
    HList![SendValue<String, End>],
    End,
  > = receive_value_from(Z, move |greeting| {
    println!("Received greetings from provider: {}", greeting);
    wait(Z, terminate())
  });
  // ANCHOR_END: hello_client_partial

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
