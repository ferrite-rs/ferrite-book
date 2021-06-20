use ferrite_session::prelude::*;

// ANCHOR: provider
fn counter(val: u64) -> Session<Rec<SendValue<u64, Z>>>
{
  fix_session(send_value(val, counter(val + 1)))
}
// ANCHOR_END: provider

// ANCHOR: client
fn counter_client() -> Session<ReceiveChannel<Rec<SendValue<u64, Z>>, End>>
{
  receive_channel(move |c1| {
    unfix_session(
      c1,
      receive_value_from(c1, move |x| {
        println!("{}", x);
        include_session(counter_client(), move |c2| {
          send_channel_to(c2, c1, forward(c2))
        })
      }),
    )
  })
}
// ANCHOR_END: client
