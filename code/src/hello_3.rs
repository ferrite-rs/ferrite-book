#![allow(dead_code)]

use ferrite_session::prelude::*;

type Hello = SendValue<String, End>;

// ANCHOR: hello_3
#[tokio::main]
async fn main()
{
  let hello_provider: Session<Hello> =
    send_value("Hello World!".to_string(), terminate());

  let hello_client: Session<ReceiveChannel<Hello, End>> =
    receive_channel(move |a| {
      receive_value_from(a, move |greeting| {
        println!("Received greetings from provider: {}", greeting);
        wait(a, terminate())
      })
    });

  let main: Session<End> = apply_channel(hello_client, hello_provider);

  run_session(main).await;
}
// ANCHOR_END: hello_3

#[test]
fn test_main()
{
  main();
}
