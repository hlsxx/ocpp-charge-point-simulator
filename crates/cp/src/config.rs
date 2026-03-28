use url::Url;

const DEFAULT_CSMS_URL: &str = "ws://localhost:3000";

pub struct CpConfig {
  csms_url: Url,
  charge_point_id: String,
  serial_number: String,
  vendor: String,
  model: String,
}

impl CpConfig {
  pub fn csms_url(&self) -> &Url {
    &self.csms_url
  }

  pub fn charge_point_id(&self) -> &str {
    &self.charge_point_id
  }

  pub fn serial_number(&self) -> &str {
    &self.serial_number
  }

  pub fn vendor(&self) -> &str {
    &self.vendor
  }

  pub fn model(&self) -> &str {
    &self.model
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
  pub fn csms_url(mut self, url_string: impl Into<String>) -> Result<Self, url::ParseError> {
    self.csms_url = Some(Url::parse(&url_string.into())?);
    Ok(self)
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
    CpConfig {
      csms_url: self
        .csms_url
        .unwrap_or_else(|| Url::parse(DEFAULT_CSMS_URL).unwrap()),
      charge_point_id: self
        .charge_point_id
        .unwrap_or_else(|| format!("CP{}", rand::random_range(100_000..999_999))),
      serial_number: self
        .serial_number
        .unwrap_or_else(|| String::from("ocpp-charge-point-simulator")),
      vendor: self.vendor.unwrap_or_else(|| String::from("ocpp-rust")),
      model: self.model.unwrap_or_else(|| String::from("ocpp-rust-v1")),
    }
  }
}
