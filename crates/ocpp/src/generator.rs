use async_trait::async_trait;
use serde_json::Value;

use crate::types::CommonConnectorStatusType;

pub trait MessageBuilder {
  fn to_call_frame(&self) -> Value;
}

#[async_trait]
pub trait MessageGenerator: Send + Sync {
  async fn heartbeat_interval(&self, _value: u32) {}
  async fn connection_timeout(&self, _value: u32) {}
  async fn reset_retries(&self, _value: u32) {}
  async fn websocket_ping_interval(&self, _value: u32) {}

  async fn meter_value_sample_interval(&self, _value: u32) {}
  async fn clock_aligned_data_interval(&self, _value: u32) {}
  async fn meter_values_sampled_data(&self, _value: String) {}
  async fn meter_values_aligned_data(&self, _value: String) {}
  async fn stop_txn_sampled_data(&self, _value: String) {}
  async fn stop_txn_aligned_data(&self, _value: String) {}

  async fn transaction_message_attempts(&self, _value: u32) {}
  async fn transaction_message_retry_interval(&self, _value: u32) {}
  async fn max_energy_on_invalid_id(&self, _value: u32) {}

  async fn authorize_remote_tx_requests(&self, _state: bool) {}
  async fn stop_transaction_on_ev_side_disconnect(&self, _state: bool) {}
  async fn stop_transaction_on_invalid_id(&self, _state: bool) {}
  async fn allow_offline_tx_for_unknown_id(&self, _state: bool) {}
  async fn local_authorize_offline(&self, _state: bool) {}
  async fn local_pre_authorize(&self, _state: bool) {}
  async fn authorization_cache_enabled(&self, _state: bool) {}

  async fn local_auth_list_enabled(&self, _state: bool) {}
  async fn local_auth_list_version(&self, _value: i32) {}
  async fn send_local_list_max_length(&self, _value: u32) {}
  async fn local_auth_list_max_length(&self, _value: u32) {}

  async fn number_of_connectors(&self, _value: u32) {}
  async fn connector_phase_rotation(&self, _value: String) {}

  async fn charge_profile_max_stack_level(&self, _value: u32) {}
  async fn charging_schedule_allowed_charging_rate_unit(&self, _value: String) {}
  async fn charging_schedule_max_periods(&self, _value: u32) {}

  async fn get_configuration_max_keys(&self, _value: u32) {}

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
  async fn change_configuration(&self) -> Value {
    todo!("ChangeConfiguration is not implemented for this OCPP version")
  }
  fn next_id(&self) -> String;
}
