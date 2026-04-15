use async_trait::async_trait;
use serde_json::Value;

use crate::types::CommonConnectorStatusType;

pub trait MessageBuilder {
  fn to_call_frame(&self) -> Value;
}

#[async_trait]
pub trait MessageGenerator: Send {
  async fn heartbeat_interval(&self, value: u32);
  async fn meter_value_sample_interval(&self, value: u32);
  async fn connection_timeout(&self, value: u32);
  async fn authorize_remote_tx_requests(&self, state: bool);
  async fn stop_transaction_on_ev_side_disconnect(&self, state: bool);
  async fn local_auth_list_enabled(&self, state: bool);
  async fn local_auth_list_version(&self, value: i32);
  async fn clock_aligned_data_interval(&self, value: u32);
  async fn transaction_message_attempts(&self, value: u32);
  async fn transaction_message_retry_interval(&self, value: u32);

  async fn boot_notification(&self) -> Value;
  async fn heartbeat(&self) -> Value;
  async fn authorize(&self, tag_id: Option<&str>) -> Value;
  async fn start_transaction(&self) -> Value;
  async fn stop_transaction(&self) -> Value;
  async fn status_notification(&self, status: CommonConnectorStatusType) -> Value;
  async fn meter_values(&self) -> Value;
  async fn diagnostics_status_notification(&self) -> Value;
  async fn firmware_status_notification(&self) -> Value;
  async fn data_transfer(&self) -> Value;
  fn next_id(&self) -> String;
}
