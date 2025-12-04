pub mod cli;
pub mod simulator;

use anyhow::Result;
use clap::Parser;
use common::Config;
use simulator::Simulator;
use tracing::Level;

use crate::cli::Args;

#[tokio::main]
async fn main() -> Result<()> {
  let args = Args::parse();
  let config = Config::try_load(args.config_path)?;

  tracing_subscriber::fmt()
    .with_max_level(if config.general.debug_mode {
      Level::DEBUG
    } else {
      Level::INFO
    })
    .with_target(true)
    .init();

  Simulator::new(args.mode, config).run().await?;

  Ok(())
}
