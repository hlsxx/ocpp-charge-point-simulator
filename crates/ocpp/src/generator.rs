use async_trait::async_trait;
use serde_json::Value;

use crate::types::CommonConnectorStatusType;

pub trait MessageBuilder {
  fn to_call_frame(&self) -> Value;
}

#[async_trait]
pub trait MessageGenerator: Send {
  // 🔌 Core / Timing
  async fn heartbeat_interval(&self, value: u32);
  async fn connection_timeout(&self, value: u32);
  async fn reset_retries(&self, value: u32);
  async fn websocket_ping_interval(&self, value: u32);

  // ⚡ Metering
  async fn meter_value_sample_interval(&self, value: u32);
  async fn clock_aligned_data_interval(&self, value: u32);
  async fn meter_values_sampled_data(&self, value: String);
  async fn meter_values_aligned_data(&self, value: String);
  async fn stop_txn_sampled_data(&self, value: String);
  async fn stop_txn_aligned_data(&self, value: String);

  // 🔄 Transaction behavior
  async fn transaction_message_attempts(&self, value: u32);
  async fn transaction_message_retry_interval(&self, value: u32);
  async fn max_energy_on_invalid_id(&self, value: u32);

  // 🔐 Authorization
  async fn authorize_remote_tx_requests(&self, state: bool);
  async fn stop_transaction_on_ev_side_disconnect(&self, state: bool);
  async fn stop_transaction_on_invalid_id(&self, state: bool);
  async fn allow_offline_tx_for_unknown_id(&self, state: bool);
  async fn local_authorize_offline(&self, state: bool);
  async fn local_pre_authorize(&self, state: bool);
  async fn authorization_cache_enabled(&self, state: bool);

  // 💳 Local Authorization List
  async fn local_auth_list_enabled(&self, state: bool);
  async fn local_auth_list_version(&self, value: i32);
  async fn send_local_list_max_length(&self, value: u32);
  async fn local_auth_list_max_length(&self, value: u32);

  // 🔌 Connector / Hardware
  async fn number_of_connectors(&self, value: u32);
  async fn connector_phase_rotation(&self, value: String);

  // ⚡ Smart Charging
  async fn charge_profile_max_stack_level(&self, value: u32);
  async fn charging_schedule_allowed_charging_rate_unit(&self, value: String);
  async fn charging_schedule_max_periods(&self, value: u32);

  // 📊 Limits / Misc
  async fn get_configuration_max_keys(&self, value: u32);

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
