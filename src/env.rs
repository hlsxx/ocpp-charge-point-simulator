use anyhow::{Context, Result};
use dotenvy::dotenv;
use std::env::var;

pub struct Env {
  pub debug_mode: bool,
  pub csms_url: String,

  // Charge point settings
  pub charge_point_serial_number: String,
  pub charge_point_vendor: String,
  pub charge_point_model: String,
  pub charge_point_clients_num: u32,
  pub charge_point_ocpp_version: String
}

impl Env {
  pub fn try_load() -> Result<Self> {
    dotenv().context("Failed to load .env file")?;

    let debug_mode = var("DEBUG_MODE")
      .context("Missing DEBUG_MODE")?
      .parse::<bool>()
      .context("DEBUG_MODE must be a boolean (true/false)")?;

    let csms_url = var("CSMS_URL").context("Missing CSMS_URL")?;
    let charge_point_serial_number =
      var("CHARGE_POINT_SERIAL_NUMBER").context("Missing CHARGE_POINT_SERIAL_NUMBER")?;
    let charge_point_vendor = var("CHARGE_POINT_VENDOR").context("Missing CHARGE_POINT_VENDOR")?;
    let charge_point_model = var("CHARGE_POINT_MODEL").context("Missing CHARGE_POINT_MODEL")?;

    let charge_point_clients_num = var("CHARGE_POINT_CLIENTS_NUM")
      .context("Missing CHARGE_POINT_CLIENTS_NUM")?
      .parse::<u32>()?;

    let charge_point_ocpp_version = var("CHARGE_POINT_OCPP_VERSION").context("Missing CHARGE_POINT_OCPP_VERSION")?;

    Ok(Self {
      debug_mode,
      csms_url,
      charge_point_serial_number,
      charge_point_vendor,
      charge_point_model,
      charge_point_clients_num,
      charge_point_ocpp_version
    })
  }
}
