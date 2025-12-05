pub mod shared_data;
use std::{fmt::Display, fs, path::Path};

use anyhow::Result;
use serde::Deserialize;
pub use shared_data::SharedData;

#[derive(Clone, Debug, Deserialize)]
pub enum OcppVersion {
  #[serde(rename = "ocpp1.6")]
  V1_6,
  #[serde(rename = "ocpp2.0.1")]
  V2_0_1,
  #[serde(rename = "ocpp2.1")]
  V2_1,
}

impl Display for OcppVersion {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let version_str = match self {
      OcppVersion::V1_6 => "ocpp1.6",
      OcppVersion::V2_0_1 => "ocpp2.0.1",
      OcppVersion::V2_1 => "ocpp2.1",
    };

    write!(f, "{}", version_str)
  }
}

impl OcppVersion {
  pub const HEADER_V1_6: &'static str = "ocpp1.6";
  pub const HEADER_V2_0_1: &'static str = "ocpp2.0.1";
  pub const HEADER_V2_1: &'static str = "ocpp2.1";

  pub fn from(header: &str) -> Option<Self> {
    match header {
      Self::HEADER_V1_6 => Some(OcppVersion::V1_6),
      Self::HEADER_V2_0_1 => Some(OcppVersion::V2_0_1),
      Self::HEADER_V2_1 => Some(OcppVersion::V2_1),
      _ => None,
    }
  }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ImplicitChargePointConfig {
  pub count: usize,
  pub prefix: String,
  pub boot_delay_range: [u64; 2],
  pub heartbeat_interval_range: [u64; 2],
  pub status_interval_range: [u64; 2],
  pub start_tx_after_range: [u64; 2],
  pub stop_tx_after_range: [u64; 2],
  pub id_tags: Vec<String>,
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
  pub auth_header: String,
  pub boot_delay_interval: u64,
  pub heartbeat_interval: u64,
  pub status_interval: u64,
  pub start_tx_after: u64,
  pub stop_tx_after: u64,
  pub id_tags: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
  pub general: GeneralConfig,
  pub charge_points: Option<Vec<ChargePointConfig>>,
  pub implicit_charge_points: Option<ImplicitChargePointConfig>,
}

impl Config {
  pub fn try_load(config_path: impl AsRef<Path>) -> Result<Self> {
    let config: Self = toml::from_str(&fs::read_to_string(config_path)?)?;
    Ok(config)
  }
}
