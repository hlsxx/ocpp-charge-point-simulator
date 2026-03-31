use clap::{Parser, ValueEnum};
use std::{fmt::Display, path::PathBuf};

#[derive(ValueEnum, Clone)]
pub enum BehaviorMode {
  #[clap(name = "dynamic")]
  Dynamic,
  #[clap(name = "idle")]
  Idle,
}

impl Display for BehaviorMode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      Self::Idle => "Idle",
      Self::Dynamic => "Dynamic",
    })
  }
}

impl BehaviorMode {
  pub fn description(&self) -> &str {
    match self {
      Self::Idle => "Mode Idle waits on commands from a CSMS",
      Self::Dynamic => "Mode Dynamic sends messages immediately after initialization",
    }
  }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
  #[arg(long, value_enum)]
  pub mode: BehaviorMode,
  #[arg(long)]
  pub config_path: PathBuf,
}
