#![cfg(test)]

use ferrite_session::*;

type Hello = SendValue < String, End >;

// ANCHOR: hello_3
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
    receive_channel ( move | provider | async move {
      receive_value_from ( provider, move | greeting | async move {
        println! ( "Received greetings from provider: {}", greeting );
          wait ( provider, terminate () )
      })
    });

  let main : Session < End > =
    apply_channel ( hello_client, hello_provider );

  run_session ( main ).await;
}
// ANCHOR_END: hello_3

#[test]
fn test_main () {
  main();
}
