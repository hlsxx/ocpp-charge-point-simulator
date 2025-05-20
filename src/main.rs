pub mod config;
pub mod ocpp;
pub mod env;

use env::Env;
use ocpp::v1_6::simulator::{WsClient, WsClientConfigBuilder};
use anyhow::Result;
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {
  let env = Env::try_load()?;

  tracing_subscriber::fmt()
    .with_max_level(if env.debug_mode { Level::DEBUG } else { Level::INFO })
    .with_target(true)
    .init();

  tracing_subscriber::fmt().init();

  let ws_client_config = WsClientConfigBuilder::new()
    .build();

  WsClient::new(ws_client_config)
    .run()
    .await?;

  Ok(())
}
