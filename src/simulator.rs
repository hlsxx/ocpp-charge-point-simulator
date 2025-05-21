use anyhow::Result;
use tracing::info;
use url::Url;

use crate::{ocpp::OcppVersion, v1_6::simulator::{WsClient, WsClientConfigBuilder}};

pub struct SimulatorConfig {
  ocpp_version: OcppVersion,
  csms_url: Url,
}

impl Default for SimulatorConfig {
  fn default() -> Self {
    Self {
      ocpp_version: OcppVersion::V1_6,
      csms_url: Url::parse("ws://localhost:3000").unwrap(),
    }
  }
}

pub struct SimulatorConfigBuilder {
  ocpp_version: Option<OcppVersion>,
  csms_url: Option<Url>,
}

impl SimulatorConfigBuilder {
  pub fn new() -> Self {
    Self {
      ocpp_version: None,
      csms_url: None,
    }
  }

  pub fn ocpp_version(mut self, ocpp_version: OcppVersion) {
    self.ocpp_version = Some(ocpp_version);
  }

  pub fn csms_url(mut self, url_string: impl Into<String>) -> Self {
    if let Ok(url) = Url::parse(&url_string.into()) {
      self.csms_url = Some(url);
    }
    self
  }

  pub fn build(self) -> SimulatorConfig {
    let config_default = SimulatorConfig::default();

    SimulatorConfig {
      ocpp_version: self.ocpp_version.unwrap_or(config_default.ocpp_version),
      csms_url: self.csms_url.unwrap_or(config_default.csms_url),
    }
  }
}

pub struct Simulator {
  config: SimulatorConfig
}

impl Simulator {
  pub fn new(config: SimulatorConfig) -> Self {
    Self {
      config
    }
  }

  pub async fn run(&self) -> Result<()> {
    info!("Simulator running...");

    match self.config.ocpp_version {
      OcppVersion::V1_6 => {
        let ws_client_config = WsClientConfigBuilder::new()
          .csms_url(self.config.csms_url.clone())
          .serial_number("TEST1-2-3")
          .model("TEST")
          .vendor("TEST")
          .build();

        WsClient::new(ws_client_config).run().await?;
      },
      OcppVersion::V2_1 => {
        todo!()
      },
      OcppVersion::V2_0_1 => {
        todo!()
      }
    }

    Ok(())
  }
}
