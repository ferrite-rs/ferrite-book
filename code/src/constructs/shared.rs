use ferrite_session::prelude::*;

// ANCHOR: provider
fn shared_counter(
  count: u64
) -> SharedSession<LinearToShared<SendValue<u64, Release>>>
{
  accept_shared_session(send_value(
    count,
    detach_shared_session(shared_counter(count + 1)),
  ))
}
// ANCHOR_END: provider

// ANCHOR: client
fn shared_counter_client(
  counter: SharedChannel<LinearToShared<SendValue<u64, Release>>>
) -> Session<End>
{
  acquire_shared_session(counter, move |c| {
    receive_value_from(c, move |x| {
      println!("{}", x);
      release_shared_session(c, terminate())
    })
  })
}
// ANCHOR_END: client

#[tokio::main]
async fn main()
{
  // ANCHOR: run
  let counter_channel = run_shared_session(shared_counter(0));
  // ANCHOR_END: run
}
