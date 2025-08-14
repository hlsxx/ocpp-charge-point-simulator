use std::sync::Arc;

use anyhow::Result;
use futures_util::future::join_all;
use tokio::task::JoinHandle;
use tracing::info;

use cp::ChargePoint;

use common::{ChargePointConfig, Config, ImplicitChargePointConfig};

use colored::Colorize;

pub struct Simulator {
  config: Config,
}

impl Simulator {
  pub fn new(config: Config) -> Self {
    info!(
      "{}",
      format!("ocpp-charge-point-simulator v{}", env!("CARGO_PKG_VERSION")).cyan()
    );

    Self { config }
  }

  pub async fn run(&self) -> Result<()> {
    info!("simulator running...");

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    let mut all_cps = self.config.charge_points.clone();

    if let Some(implicit_cps) = &self.config.implicit_charge_points {
      let generated = Self::generate_implicit_cps(implicit_cps);
      all_cps.extend(generated);
    }

    let general_config = Arc::new(self.config.general.clone());
    for cp_config in all_cps {
      let general_config_clone = general_config.clone();

      let handle = tokio::spawn(async move {
        let mut charger = ChargePoint::new(general_config_clone, cp_config);

        if let Err(e) = charger.run().await {
          eprintln!("Client failed: {:?}", e);
        }
      });

      handles.push(handle);
    }

    join_all(handles).await;

    Ok(())
  }

  /// Generates implicit charge points from the config file.
  fn generate_implicit_cps(cfg: &ImplicitChargePointConfig) -> Vec<ChargePointConfig> {
    (0..cfg.count)
      .map(|i| ChargePointConfig {
        id: format!("{}{:06}", cfg.prefix, i),
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
