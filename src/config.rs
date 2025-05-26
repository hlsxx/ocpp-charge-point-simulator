use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GeneralConfig {
  pub server_url: String,
  pub ocpp_version: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
  pub general: GeneralConfig,
  pub charge_points: Vec<ChargePointConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChargePointConfig {
  pub id: String,
  pub boot_delay_ms: u64,
  pub heartbeat_interval: u64,
  pub status_interval: u64,
  pub start_tx_after: u64,
  pub stop_tx_after: u64,
}
