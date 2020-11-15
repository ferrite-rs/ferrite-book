#![cfg(test)]

mod hello_1 {

// ANCHOR: hello_1
use ferrite_session::*;

type Hello = SendValue < String, End >;

#[async_std::main]
async fn main () {
  // ANCHOR: hello_provider
  let hello_provider : Session < Hello > =
    send_value! (
      "Hello World!".to_string(),
      terminate! () );
  // ANCHOR_END: hello_provider

  // ANCHOR: run_session_with_result
  let result : String =
    run_session_with_result ( hello_provider ).await;

  println!("{}", result);
  // ANCHOR_END: run_session_with_result
}
// ANCHOR_END: hello_1

#[test]
fn test_main () {
  main();
}

}

mod hello_2 {

// ANCHOR: hello_2
use ferrite_session::*;

type Hello = SendValue < String, End >;

#[async_std::main]
async fn main () {
  let hello_provider : Session < Hello > =
    send_value! (
      "Hello World!".to_string(),
      terminate() );

  // ANCHOR: hello_client
  let hello_client :
    Session <
      ReceiveChannel < Hello, End >
    > =
    receive_channel! ( provider => {
      receive_value_from! ( provider, greeting => {
        println! ( "Received greetings from provider: {}", greeting );

        wait! ( provider, terminate! () )
      })
    });
  // ANCHOR_END: hello_client

  // ANCHOR: apply_channel
  let main : Session < End > =
    apply_channel ( hello_client, hello_provider );
  // ANCHOR_END: apply_channel

  // ANCHOR: run_session
  run_session ( main ).await;
  // ANCHOR_END: run_session
}
// ANCHOR_END: hello_2

#[test]
fn test_main () {
  main();
}

}