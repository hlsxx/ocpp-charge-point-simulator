use clap::{Parser, ValueEnum};
use std::fmt::Display;

#[derive(ValueEnum, Clone)]
pub enum BehaviorMode {
  #[clap(name = "dynamic")]
  Dynamic,

  #[clap(name = "idle")]
  Idle,
}

impl Display for BehaviorMode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Idle => f.write_fmt(format_args!("Idle")),
      Self::Dynamic => f.write_fmt(format_args!("Dynamic")),
    }
  }
}

impl BehaviorMode {
  pub fn description(&self) -> String {
    match self {
      Self::Idle => String::from("Mode Idle waits on commands from a CSMS"),
      Self::Dynamic => String::from("Mode Dynamic sends messages immediately after initialization"),
    }
  }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
  /// Mode of a behavior
  #[arg(long, value_enum)]
  pub mode: BehaviorMode,

  /// Path to a config file
  #[arg(long)]
  pub config_path: String,
}
