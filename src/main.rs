pub mod config;
pub mod env;
pub mod message;
pub mod ocpp;
pub mod simulator;
pub mod v1_6;
pub mod v2_0_1;
pub mod v2_1;
pub mod charger;

use anyhow::Result;
use config::Config;
use simulator::Simulator;
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {
  let config = Config::try_load()?;

  tracing_subscriber::fmt()
    .with_max_level(if config.general.debug_mode {
      Level::DEBUG
    } else {
      Level::INFO
    })
    .with_target(true)
    .init();

  Simulator::new(config).run().await?;

  Ok(())
}
