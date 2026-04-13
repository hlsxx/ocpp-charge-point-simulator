use std::sync::Arc;

use anyhow::Result;
use futures_util::future::join_all;
use tokio::task::JoinHandle;
use tracing::{error, info};

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
    Self { mode, config }
  }

  pub async fn run(&mut self) -> Result<()> {
    info!(
      "ocpp-charge-point-simulator v{}",
      env!("CARGO_PKG_VERSION").cyan()
    );
    info!(
      "{} [{}]",
      self.mode.to_string().purple(),
      self.mode.description()
    );
    info!("simulator running...");

    if let Some(implicit_cps) = &self.config.implicit_charge_points {
      self
        .config
        .charge_points
        .extend(Self::generate_implicit_cps(implicit_cps));
    }

    let general_config = Arc::new(self.config.general.clone());

    let handles = self
      .config
      .charge_points
      .iter()
      .map(|cp_config| self.spawn_cp(Arc::clone(&general_config), cp_config.clone()))
      .collect::<Vec<JoinHandle<()>>>();

    for res in join_all(handles).await {
      if let Err(err) = res {
        error!("Charge point task error: {:?}", err);
      }
    }

    Ok(())
  }

  fn spawn_cp(
    &self,
    general_config: Arc<common::GeneralConfig>,
    cp_config: ChargePointConfig,
  ) -> JoinHandle<()> {
    match self.mode {
      BehaviorMode::Idle => tokio::spawn(async move {
        if let Err(e) = ChargePointIdle::new(general_config, cp_config).run().await {
          error!("Charge point [{}] failed: {:?}", BehaviorMode::Idle, e);
        }
      }),
      BehaviorMode::Dynamic => tokio::spawn(async move {
        if let Err(e) = ChargePointDynamic::new(general_config, cp_config)
          .run()
          .await
        {
          error!("Charge point [{}] failed: {:?}", BehaviorMode::Dynamic, e);
        }
      }),
    }
  }

  fn generate_implicit_cps(cfg: &ImplicitChargePointConfig) -> Vec<ChargePointConfig> {
    (0..cfg.count)
      .map(|i| ChargePointConfig {
        id: format!("{}{:06}", cfg.prefix, i),
        model: format!("model-name-{:06}", i),
        vendor: format!("vendor-name-{:06}", i),
        auth_header: String::new(),
        boot_delay_interval: rand::random_range(cfg.boot_delay_range[0]..=cfg.boot_delay_range[1]),
        heartbeat_interval: rand::random_range(
          cfg.heartbeat_interval_range[0]..=cfg.heartbeat_interval_range[1],
        ),
        txn_meter_values_interval: 5,
        txn_meter_values_max_count: 60,
        status_interval: rand::random_range(
          cfg.status_interval_range[0]..=cfg.status_interval_range[1],
        ),
        start_tx_after: rand::random_range(
          cfg.start_tx_after_range[0]..=cfg.start_tx_after_range[1],
        ),
        stop_tx_after: rand::random_range(cfg.stop_tx_after_range[0]..=cfg.stop_tx_after_range[1]),
        id_tag: cfg.id_tag.clone(),
      })
      .collect()
  }
}
