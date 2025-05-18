pub mod config;
pub mod ocpp;

use ocpp::v1_6::simulator::{WsClient, WsClientConfigBuilder};

#[tokio::main]
async fn main() {
  let ws_client_config = WsClientConfigBuilder::new()
    .build();

  WsClient::new(ws_client_config)
    .run()
    .await;
}
