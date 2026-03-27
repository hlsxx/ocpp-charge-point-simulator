pub mod cli;
pub mod simulator;

use anyhow::Result;
use clap::Parser;
use common::Config;
use simulator::Simulator;
use tracing::Level;

use crate::cli::Args;

fn init_tracing(debug_mode: bool) {
  let level = if debug_mode {
    Level::DEBUG
  } else {
    Level::INFO
  };
  tracing_subscriber::fmt()
    .with_max_level(level)
    .with_target(true)
    .init();
}

#[tokio::main]
async fn main() -> Result<()> {
  let args = Args::parse();
  let config = Config::try_load(args.config_path)?;
  init_tracing(config.general.debug_mode);
  Simulator::new(args.mode, config).run().await
}
