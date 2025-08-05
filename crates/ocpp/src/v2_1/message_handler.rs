use crate::messsage_handler::{OcppMessageFrameType, OcppMessageHandler};
use anyhow::Result;
use async_trait::async_trait;
pub struct MessageHandler {}

impl MessageHandler {
  pub fn new() -> Self {
    Self {}
  }
}

#[async_trait]
impl OcppMessageHandler for MessageHandler {
  fn parse_ocpp_message(&self, text: &str) -> Result<OcppMessageFrameType> {
    !unimplemented!()
  }

  async fn handle_text_message(&mut self, text: &str) -> Result<Option<String>> {
    !unimplemented!()
  }
}
