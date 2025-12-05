use std::sync::Arc;

use anyhow::Result;
use futures_util::future::join_all;
use tokio::task::JoinHandle;
use tracing::info;

use colored::Colorize;
use common::{ChargePointConfig, Config, ImplicitChargePointConfig};
use cp::{dynamic::ChargePointDynamic, idle::ChargePointIdle};

use crate::cli::BehaviorMode;

pub struct Simulator {
  mode: BehaviorMode,
  config: Config,
}

impl Simulator {
  pub fn new(mode: BehaviorMode, config: Config) -> Self {
    info!(
      "{}",
      format!("ocpp-charge-point-simulator v{}", env!("CARGO_PKG_VERSION")).cyan(),
    );

    info!(
      "{} [{}]",
      format!("{}", mode).purple(),
      format!("{}", mode.description())
    );

    Self { mode, config }
  }

  /// Reads the configured charge point definitions and starts virtual charge points.
  ///
  /// Each charge point is spawned as a separate asynchronous task using `tokio::spawn`.
  /// Both explicit charge points from the configuration file and any generated
  /// implicit charge points are included.
  pub async fn run(&self) -> Result<()> {
    info!("simulator running...");

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    let mut all_cps = self.config.charge_points.clone().unwrap_or_default();

    if let Some(implicit_cps) = &self.config.implicit_charge_points {
      let generated = Self::generate_implicit_cps(implicit_cps);
      all_cps.extend(generated);
    }

    let general_config = Arc::new(self.config.general.clone());
    for cp_config in all_cps {
      let general_config = general_config.clone();

      let handle = match self.mode {
        BehaviorMode::Idle => tokio::spawn(async move {
          let mut cp_idle = ChargePointIdle::new(general_config, cp_config);

          if let Err(e) = cp_idle.run().await {
            eprintln!("Charge point [Idle] failed: {:?}", e);
          }
        }),
        BehaviorMode::Dynamic => tokio::spawn(async move {
          let mut cp_dynamic = ChargePointDynamic::new(general_config, cp_config);

          if let Err(e) = cp_dynamic.run().await {
            eprintln!("Charge point [Dynamic] failed: {:?}", e);
          }
        }),
      };

      handles.push(handle);
    }

    join_all(handles).await;

    Ok(())
  }

  /// Generates a list of charge point configurations from the given implicit config.
  fn generate_implicit_cps(cfg: &ImplicitChargePointConfig) -> Vec<ChargePointConfig> {
    (0..cfg.count)
      .map(|i| ChargePointConfig {
        id: format!("{}{:06}", cfg.prefix, i),
        auth_header: String::new(),
        boot_delay_interval: rand::random_range(cfg.boot_delay_range[0]..=cfg.boot_delay_range[1]),
        heartbeat_interval: rand::random_range(
          cfg.heartbeat_interval_range[0]..=cfg.heartbeat_interval_range[1],
        ),
        status_interval: rand::random_range(
          cfg.status_interval_range[0]..=cfg.status_interval_range[1],
        ),
        start_tx_after: rand::random_range(
          cfg.start_tx_after_range[0]..=cfg.start_tx_after_range[1],
        ),
        stop_tx_after: rand::random_range(cfg.stop_tx_after_range[0]..=cfg.stop_tx_after_range[1]),
        id_tags: cfg.id_tags.clone(),
      })
      .collect()
  }
}
