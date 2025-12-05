use anyhow::Result;
use common::{ChargePointConfig, GeneralConfig};
use futures::SinkExt;
use ocpp::create_ocpp_handlers;
use std::sync::Arc;
use tungstenite::Message;

use futures_util::StreamExt;
use tracing::info;

use super::core::ChargePointClient;

/// An `idle mode` charge point
///
/// Represents an `idle` mode charge point
pub struct ChargePointIdle {
  /// General config
  general_config: Arc<GeneralConfig>,

  // Specific chage point config
  config: ChargePointConfig,
}

impl ChargePointIdle {
  pub fn new(general_config: Arc<GeneralConfig>, config: ChargePointConfig) -> Self {
    Self {
      general_config,
      config,
    }
  }

  /// Runs a charge point in `idle mode` that sends messages at specific intervals to the CSMS server.
  /// In idle mode, the charge point sends and also listening for messages.
  pub async fn run(&mut self) -> Result<()> {
    let ws_stream = ChargePointClient::connect(&self.general_config, &self.config).await?;
    let (mut ws_tx, mut _ws_rx) = ws_stream.split();

    let (msg_generator, mut msg_handler) =
      create_ocpp_handlers(self.general_config.ocpp_version.clone());

    let heartbeat = msg_generator.heartbeat().await;

    ws_tx
      .send(Message::Text(heartbeat.to_string().into()))
      .await
      .unwrap();

    info!("Client shutdown");

    Ok(())
  }
}
