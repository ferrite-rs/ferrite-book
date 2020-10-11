#![cfg(test)]

mod hello_1 {

// ANCHOR: hello_1
use ferrite::*;

#[async_std::main]
async fn main () {
  let session :
    Session <
      SendValue < String, End >
    > =
    send_value! (
      "Hello World!".to_string(),
      terminate! () );

  let result =
    run_session_with_result ( session ).await;

  println!("{}", result);
  // ANCHOR_END: hello_1

  assert_eq!(result, "Hello World!");
}

#[test]
fn test_main () {
  main();
}

}

mod hello_2 {

use ferrite::*;

#[async_std::main]
async fn main () {

// ANCHOR: hello_2
use std::time::Duration;
use async_std::task::sleep;

let hello_provider :
  Session <
    SendValue < String, End >
  > =
  send_value! (
    {
      sleep(Duration::from_secs(2)).await;
      "Hello World!".to_string()
    },
    terminate() );

let hello_client :
  Session <
    ReceiveChannel <
      SendValue < String, End >,
      End
    >
  > =
  receive_channel! ( provider => {
    receive_value_from! ( provider, greeting => {
      println! ( "Received greetings from provider: {}", greeting );

      wait! ( provider,
        terminate! () )
    })
  });

let session : Session < End > =
  apply_channel ( hello_client, hello_provider );

run_session ( session ).await;
// ANCHOR_END: hello_2

}

}