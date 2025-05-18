pub mod config;
pub mod ocpp;

use ocpp::v1_6::simulator::{WsClient, WsClientConfigBuilder};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let ws_client_config = WsClientConfigBuilder::new()
    .build();

  WsClient::new(ws_client_config)
    .run()
    .await?;

  Ok(())
}
