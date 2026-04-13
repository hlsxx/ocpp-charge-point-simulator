use async_trait::async_trait;
use serde_json::Value;

use crate::types::CommonConnectorStatusType;

pub trait MessageBuilder {
  fn to_call_frame(&self) -> Value;
}

#[async_trait]
pub trait MessageGenerator: Send {
  async fn boot_notification(&self) -> Value;
  async fn heartbeat(&self) -> Value;
  async fn authorize(&self) -> Value;
  async fn start_transaction(&self, tag_id: Option<&str>) -> Value;
  async fn stop_transaction(&self) -> Value;
  async fn status_notification(&self, status: CommonConnectorStatusType) -> Value;
  async fn meter_values(&self) -> Value;
  async fn diagnostics_status_notification(&self) -> Value;
  async fn firmware_status_notification(&self) -> Value;
  async fn data_transfer(&self) -> Value;
  fn next_id(&self) -> String;
}
