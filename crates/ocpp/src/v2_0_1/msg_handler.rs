#![allow(unused)]
use crate::{
  msg_handler::{MessageFrameType, MessageHandler},
  types::CommonOcppAction,
};

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

pub struct V201MessageHandler {}

impl V201MessageHandler {
  pub fn new() -> Self {
    Self {}
  }
}

#[async_trait]
impl MessageHandler for V201MessageHandler {
  fn parse_ocpp_message(&self, text: &str) -> Result<MessageFrameType> {
    !unimplemented!()
  }

  async fn handle_text_message(&mut self, text: &str) -> Result<Option<String>> {
    !unimplemented!()
  }

  async fn handle_call_result(
    &self,
    msg_id: &str,
    payload: &Value,
  ) -> Result<Option<CommonOcppAction>> {
    !unimplemented!()
  }
}
