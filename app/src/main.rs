pub mod simulator;

use anyhow::Result;
use simulator::Simulator;
use tracing::Level;
use common::Config;

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
