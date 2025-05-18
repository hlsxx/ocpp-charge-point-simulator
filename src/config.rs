use std::env;
use dotenvy::dotenv;
use crate::ocpp::v1_6::simulator::WsClientConfigBuilder;

pub fn load_ws_client_config_from_env() -> WsClientConfigBuilder {
  dotenv().ok();

  let mut builder = WsClientConfigBuilder::new();

  if let Ok(url) = env::var("CSMS_URL") {
    builder = builder.csms_url(url);
  }

  if let Ok(id) = env::var("CHARGE_POINT_ID") {
    builder = builder.charge_point_id(id);
  }

  if let Ok(vendor) = env::var("CHARGE_POINT_VENDOR") {
    builder = builder.vendor(vendor);
  }

  if let Ok(model) = env::var("CHARGE_POINT_MODEL") {
    builder = builder.model(model);
  }

  builder
}
