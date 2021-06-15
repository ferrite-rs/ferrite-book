use ferrite_session::prelude::*;

#[tokio::main]
async fn main()
{
  // ANCHOR: show_number
  let show_number: Session<ReceiveValue<i32, End>> = receive_value(|x| {
    println!("The magic number is: {}", x);
    terminate()
  });
  // ANCHOR_END: show_number

  // ANCHOR: main
  let main: Session<End> = include_session(show_number, |chan| {
    send_value_to(chan, 42, wait(chan, terminate()))
  });

  run_session(main).await;
  // ANCHOR_END: main
}
