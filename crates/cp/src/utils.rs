use tungstenite::Message;

pub struct MessageBuilder;

impl MessageBuilder {
  pub fn new_text<T: ToString>(msg: T) -> Message {
    Message::Text(msg.to_string().into())
  }
}
