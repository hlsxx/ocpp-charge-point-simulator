#![cfg(feature = "ocpp2_0_1")]
#![allow(unused)]
use crate::{
  handler::{MessageFrameType, MessageHandler},
  types::CommonOcppResponse,
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
  async fn parse_raw_ocpp_msg(&self, _text: &str) -> Result<MessageFrameType> {
    todo!("OCPP 2.0.1 message parsing is not implemented")
  }

  async fn handle_text_message(&mut self, _text: &str) -> Result<Option<String>> {
    todo!("OCPP 2.0.1 message handling is not implemented")
  }

  async fn handle_call_result(
    &self,
    msg_id: &str,
    payload: &Value,
  ) -> Result<Option<CommonOcppResponse>> {
    todo!("OCPP 2.0.1 call-result handling is not implemented")
  }
}
