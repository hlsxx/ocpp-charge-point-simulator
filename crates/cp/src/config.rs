use url::Url;

pub struct CpConfig {
  csms_url: Url,
  charge_point_id: String,
  serial_number: String,
  vendor: String,
  model: String,
}

impl Default for CpConfig {
  fn default() -> Self {
    Self {
      csms_url: Url::parse("ws://localhost:3000").unwrap(),
      charge_point_id: format!("CP{}", rand::random_range(100_000..999_999)),
      serial_number: String::from("ocpp-charge-point-simulator"),
      vendor: String::from("ocpp-rust"),
      model: String::from("ocpp-rust-v1"),
    }
  }
}

#[derive(Default)]
pub struct CpConfigBuilder {
  csms_url: Option<Url>,
  charge_point_id: Option<String>,
  serial_number: Option<String>,
  vendor: Option<String>,
  model: Option<String>,
}

impl CpConfigBuilder {
  pub fn csms_url(mut self, url_string: impl Into<String>) -> Self {
    if let Ok(url) = Url::parse(&url_string.into()) {
      self.csms_url = Some(url);
    }
    self
  }

  pub fn charge_point_id(mut self, id: impl Into<String>) -> Self {
    self.charge_point_id = Some(id.into());
    self
  }

  pub fn serial_number(mut self, serial_number: impl Into<String>) -> Self {
    self.serial_number = Some(serial_number.into());
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

  pub fn build(self) -> CpConfig {
    let config_default = CpConfig::default();

    CpConfig {
      csms_url: self.csms_url.unwrap_or(config_default.csms_url),
      charge_point_id: self
        .charge_point_id
        .unwrap_or(config_default.charge_point_id),
      serial_number: self.serial_number.unwrap_or(config_default.serial_number),
      vendor: self.vendor.unwrap_or(config_default.vendor),
      model: self.model.unwrap_or(config_default.model),
    }
  }
}
