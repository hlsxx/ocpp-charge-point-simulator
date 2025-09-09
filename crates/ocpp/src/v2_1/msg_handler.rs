use crate::msg_handler::{OcppMessageFrameType, OcppMessageHandler};
use anyhow::Result;
use async_trait::async_trait;
pub struct MessageHandler {}

#[allow(clippy::new_without_default)]
impl MessageHandler {
  pub fn new() -> Self {
    Self {}
  }
}

#[async_trait]
impl OcppMessageHandler for MessageHandler {
  #[allow(unused)]
  fn parse_ocpp_message(&self, text: &str) -> Result<OcppMessageFrameType> {
    !unimplemented!()
  }

  #[allow(unused)]
  async fn handle_text_message(&mut self, text: &str) -> Result<Option<String>> {
    !unimplemented!()
  }
}
