use anyhow::Result;
use common::{ChargePointConfig, GeneralConfig};
use std::sync::Arc;

use futures_util::StreamExt;
use tracing::info;

use crate::ChargePointClient;

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
    let (mut ws_tx, mut ws_rx) = ws_stream.split();

    info!("Client shutdown");

    Ok(())
  }
}
