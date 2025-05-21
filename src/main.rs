pub mod env;
pub mod message_generator;
pub mod ocpp;

use anyhow::Result;
use env::Env;
use ocpp::v1_6::simulator::{WsClient, WsClientConfigBuilder};
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {
  let env = Env::try_load()?;

  tracing_subscriber::fmt()
    .with_max_level(if env.debug_mode {
      Level::DEBUG
    } else {
      Level::INFO
    })
    .with_target(true)
    .init();

  let ws_client_config = WsClientConfigBuilder::new()
    .csms_url(env.csms_url)
    .serial_number(env.charge_point_serial_number)
    .model(env.charge_point_model)
    .vendor(env.charge_point_vendor)
    .build();

  WsClient::new(ws_client_config).run().await?;

  Ok(())
}
