use tracing::info;

pub struct WsClientConfig {
  csms_url: String,
  charge_point_id: String,
  vendor: String,
  model: String,
}

impl Default for WsClientConfig {
  fn default() -> Self {
    Self {
      csms_url: String::from("ws://localhost:3000"),
      charge_point_id: String::from("ocpp-charge-point-simulator"),
      vendor: String::from("ocpp-rust"),
      model: String::from("ocpp-rust-v1"),
    }
  }
}

pub struct WsClientConfigBuilder {
  csms_url: Option<String>,
  charge_point_id: Option<String>,
  vendor: Option<String>,
  model: Option<String>,
}

impl WsClientConfigBuilder {
  pub fn new() -> Self {
    Self {
      csms_url: None,
      charge_point_id: None,
      vendor: None,
      model: None
    }
  }

  pub fn csms_url(mut self, url: impl Into<String>) -> Self { self.csms_url = Some(url.into()); self }

  pub fn charge_point_id(mut self, id: impl Into<String>) -> Self {
    self.charge_point_id = Some(id.into());
    self
  }

  pub fn vendor(mut self, vendor: impl Into<String>) -> Self {
    self.vendor = Some(vendor.into());
    self
  }

  pub fn model(mut self, model: impl Into<String>) -> Self {
    self.model = Some(model.into());
    self
  }

  pub fn build(self) -> WsClientConfig {
    let config_default = WsClientConfig::default();

    WsClientConfig {
      csms_url: self.csms_url.unwrap_or(config_default.csms_url),
      charge_point_id: self.charge_point_id.unwrap_or(config_default.charge_point_id),
      vendor: self.vendor.unwrap_or(config_default.vendor),
      model: self.model.unwrap_or(config_default.model),
    }
  }
}

pub struct WsClient {
  config: WsClientConfig
}

impl WsClient {
  pub fn new(config: WsClientConfig) -> Self {
    Self {
      config
    }
  }

  pub async fn run(&mut self) {
    info!(url = %self.config.csms_url, id = %self.config.charge_point_id, "Starting WsClient");
  }
}

