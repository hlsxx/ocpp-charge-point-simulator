use std::fs;

use anyhow::Result;
use serde::Deserialize;

use crate::ocpp::OcppVersion;

#[derive(Debug, Deserialize, Clone)]
pub struct ImplicitChargePointConfig {
  pub count: usize,
  pub prefix: String,
  pub boot_delay_range: [u64; 2],
  pub heartbeat_interval_range: [u64; 2],
  pub status_interval_range: [u64; 2],
  pub start_tx_after_range: [u64; 2],
  pub stop_tx_after_range: [u64; 2],
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeneralConfig {
  pub debug_mode: bool,
  pub server_url: String,
  pub ocpp_version: OcppVersion,
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

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
  pub general: GeneralConfig,
  pub charge_points: Vec<ChargePointConfig>,
  pub implicit_charge_points: Option<ImplicitChargePointConfig>,
}

impl Config {
  pub fn try_load() -> Result<Self> {
    let config: Self = toml::from_str(&fs::read_to_string("config.toml")?)?;
    Ok(config)
  }
}
