use std::sync::Arc;

use anyhow::Result;
use futures_util::future::join_all;
use tokio::task::JoinHandle;
use tracing::info;
use url::Url;

use crate::{
  ocpp::OcppVersion,
  ws_client::{WsClient, WsClientConfigBuilder},
};
use colored::Colorize;

pub struct SimulatorConfig {
  ocpp_version: OcppVersion,
  csms_url: Url,
  clients_num: u32
}

impl Default for SimulatorConfig {
  fn default() -> Self {
    Self {
      ocpp_version: OcppVersion::V1_6,
      csms_url: Url::parse("ws://localhost:3000").unwrap(),
      clients_num: 1
    }
  }
}

pub struct SimulatorConfigBuilder {
  ocpp_version: Option<OcppVersion>,
  csms_url: Option<Url>,
  clients_num: Option<u32>
}

impl SimulatorConfigBuilder {
  pub fn new() -> Self {
    Self {
      ocpp_version: None,
      csms_url: None,
      clients_num: None
    }
  }

  pub fn ocpp_version(mut self, ocpp_version: OcppVersion) -> Self {
    self.ocpp_version = Some(ocpp_version);
    self
  }

  pub fn csms_url(mut self, url_string: impl Into<String>) -> Self {
    if let Ok(url) = Url::parse(&url_string.into()) {
      self.csms_url = Some(url);
    }
    self
  }

  pub fn clients_num(mut self, clients_num: u32) -> Self {
    self.clients_num = Some(clients_num);
    self
  }

  pub fn build(self) -> SimulatorConfig {
    let config_default = SimulatorConfig::default();

    SimulatorConfig {
      ocpp_version: self.ocpp_version.unwrap_or(config_default.ocpp_version),
      csms_url: self.csms_url.unwrap_or(config_default.csms_url),
      clients_num: self.clients_num.unwrap_or(config_default.clients_num),
    }
  }
}

pub struct Simulator {
  config: SimulatorConfig,
}

impl Simulator {
  pub fn new(config: SimulatorConfig) -> Self {
    info!(
      "{}",
      format!("ocpp-charge-point-simulator v{}", env!("CARGO_PKG_VERSION")).cyan()
    );

    Self { config }
  }

  pub async fn run(&self) -> Result<()> {
    info!("simulator running...");

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    match self.config.ocpp_version {
      OcppVersion::V1_6 => {
        for i in 0..self.config.clients_num {
          let ws_client_config = Arc::new(WsClientConfigBuilder::new()
            .csms_url(self.config.csms_url.clone())
            .serial_number(format!("a-b-c-d-e-{}", i))
            //.model("TEST")
            //.vendor("TEST")
            .build());

          let ws_client_config = ws_client_config.clone();

          let handle = tokio::spawn(async move {
            let mut client = WsClient::new(ws_client_config.clone());

            if let Err(e) = client.run().await {
              eprintln!("Client {} failed: {:?}", i, e);
            }
          });

          handles.push(handle);
        }
      }
      OcppVersion::V2_1 => {
        todo!()
      }
      OcppVersion::V2_0_1 => {
        todo!()
      }
    }

    join_all(handles).await;

    Ok(()) }
}
