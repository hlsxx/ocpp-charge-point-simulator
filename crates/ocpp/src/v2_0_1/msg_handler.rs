use crate::msg_handler::{MessageFrameType, MessageHandler};

use anyhow::Result;
use async_trait::async_trait;

pub struct V201MessageHandler {}

impl V201MessageHandler {
  pub fn new() -> Self {
    Self {}
  }
}

#[async_trait]
impl MessageHandler for V201MessageHandler {
  #[allow(unused)]
  fn parse_ocpp_message(&self, text: &str) -> Result<MessageFrameType> {
    !unimplemented!()
  }

  #[allow(unused)]
  async fn handle_text_message(&mut self, text: &str) -> Result<Option<String>> {
    !unimplemented!()
  }
}
