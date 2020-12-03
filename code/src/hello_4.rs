#![cfg(test)]

use ferrite_session::*;

type Hello = SendValue < String, End >;

// ANCHOR: hello_4
#[async_std::main]
async fn main () {
  let hello_provider : Session < Hello > =
    send_value (
      "Hello World!".to_string(),
      terminate () );

  let hello_client :
    Session <
      ReceiveChannel < Hello, End >
    > =
    receive_channel ( async move | provider | {
      receive_value_from ( provider, async move | greeting | {
        println! ( "Received greetings from provider: {}", greeting );
          wait ( provider, terminate () )
      })
    });

  let main : Session < End > =
    apply_channel ( hello_client, hello_provider );

  run_session ( main ).await;
}
// ANCHOR_END: hello_4

#[test]
fn test_main () {
  main();
}
