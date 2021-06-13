#![allow(dead_code)]

// ANCHOR: hello_1
use ferrite_session::prelude::*;

type Hello = SendValue<String, End>;

#[tokio::main]
async fn main()
{

// ANCHOR: hello_provider
let hello_provider: Session<Hello> =
  send_value("Hello World!".to_string(), terminate());
// ANCHOR_END: hello_provider

// ANCHOR: run_session_with_result
let result: String = run_session_with_result(hello_provider).await;

println!("{}", result);
// ANCHOR_END: run_session_with_result

}
// ANCHOR_END: hello_1

#[test]
fn test_main()
{
  main();
}

mod hello_2
{}
