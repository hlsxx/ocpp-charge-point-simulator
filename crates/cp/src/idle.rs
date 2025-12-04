use anyhow::Result;
use common::{ChargePointConfig, GeneralConfig};
use std::sync::Arc;

use futures_util::StreamExt;
use tracing::info;

use super::core::ChargePointClient;

pub struct ChargePointIdle {
  general_config: Arc<GeneralConfig>,
  config: ChargePointConfig,
}

impl ChargePointIdle {
  pub fn new(general_config: Arc<GeneralConfig>, config: ChargePointConfig) -> Self {
    Self {
      general_config,
      config,
    }
  }

  pub async fn run(&mut self) -> Result<()> {
    let ws_stream = ChargePointClient::connect(&self.general_config, &self.config).await?;
    let (mut _ws_tx, mut _ws_rx) = ws_stream.split();

    info!("Client shutdown");

    Ok(())
  }
}
