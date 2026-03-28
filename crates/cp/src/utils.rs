use std::fmt::Display;

use tungstenite::Message;

pub struct MessageBuilder;

impl MessageBuilder {
  // Use just as a namespace
  #[allow(unused)]
  fn new() -> Self {
    unreachable!()
  }

  pub fn text(msg: impl Display) -> Message {
    Message::Text(msg.to_string().into())
  }
}
