#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(clippy::diverging_sub_expression)]

use ferrite_session::prelude::*;

#[tokio::main]
async fn main()
{
  {
    // ANCHOR: hello_hole_1
    let hello_provider: Session<SendValue<String, End>> = todo!() as _;
    // ANCHOR_END: hello_hole_1
  }

  {
    // ANCHOR: hello_hole_2
    let hello_provider: Session<SendValue<String, End>> =
      send_value("Hello World!".to_string(), todo!());
    // ANCHOR_END: hello_hole_2
  }

  {
    let hello_provider: Session<SendValue<String, End>> =
      send_value("Hello World!".to_string(), todo!() as Session<End>);
  }
  {
    let hello_client: Session<ReceiveChannel<SendValue<String, End>, End>> =
      receive_channel(move |a| {
        todo!() as PartialSession<HList![_], End>
      });
  }
  {
    let hello_client: Session<ReceiveChannel<SendValue<String, End>, End>> =
      receive_channel(move |a| {
        todo!() as PartialSession<HList![SendValue<String, _>], End>
      });
  }
  {
    let hello_client: Session<ReceiveChannel<SendValue<String, End>, End>> =
      receive_channel(move |a| {
        receive_value_from(a, |val| todo!() as PartialSession<HList![End], End>)
      });
  }
  {
    let hello_client: Session<ReceiveChannel<SendValue<String, End>, End>> =
      receive_channel(move |a| {
        receive_value_from(a, move |val| {
          wait(a, todo!() as PartialSession<HList![Empty], End>)
        })
      });
  }
}

#[test]
fn test_main() {}
