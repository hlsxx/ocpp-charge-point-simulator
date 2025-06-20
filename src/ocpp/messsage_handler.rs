use anyhow::Result;
use async_trait::async_trait;

use super::types::OcppMessageFrameType;

#[async_trait]
pub trait OcppMessageHandler: Send + Sync {
  async fn handle_text_message(&mut self, text: &str) -> Result<Option<String>>;
  fn parse_ocpp_message(&self, text: &str) -> Result<OcppMessageFrameType>;
}
