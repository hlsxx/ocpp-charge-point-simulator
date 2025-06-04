use serde::Serialize;
use serde_json::Value;

pub trait MessageGeneratorTrait {
  type OcppAction;

  type BootNotification;
  type Heartbeat;
  type Authorize;
  type StartTransaction;
  type StopTransaction;
  type StatusNotification;
  type MeterValues;
  type DiagnosticsStatusNotification;
  type FirmwareStatusNotification;
  type DataTransfer;

  fn boot_notification(&self) -> Self::BootNotification;
  fn heartbeat(&self) -> Self::Heartbeat;
  fn authorize(&self) -> Self::Authorize;
  fn start_transaction(&self) -> Self::StartTransaction;
  fn stop_transaction(&self) -> Self::StopTransaction;
  fn status_notification(&self) -> Self::StatusNotification;
  fn meter_values(&self) -> Self::MeterValues;
  fn diagnostics_status_notification(&self) -> Self::DiagnosticsStatusNotification;
  fn firmware_status_notification(&self) -> Self::FirmwareStatusNotification;
  fn data_transfer(&self) -> Self::DataTransfer;

  fn next_id(&self) -> String;
  fn to_frame<T: Serialize>(&self, action: Self::OcppAction, payload: T) -> Value;
}

pub struct MessageGeneratorConfig {
  pub(crate) serial_number: String,
  pub(crate) vendor: String,
  pub(crate) model: String,
  pub(crate) id_tag: String,
}

impl Default for MessageGeneratorConfig {
  fn default() -> Self {
    Self {
      serial_number: String::from("ocpp-charge-point-simulator"),
      vendor: String::from("ocpp-rust"),
      model: String::from("ocpp-rust-v1"),
      id_tag: String::from("abcdefgh"),
    }
  }
}

pub struct MessageGeneratorConfigBuilder {
  serial_number: Option<String>,
  vendor: Option<String>,
  model: Option<String>,
  id_tag: Option<String>,
}

impl MessageGeneratorConfigBuilder {
  pub fn new() -> Self {
    Self {
      serial_number: None,
      vendor: None,
      model: None,
      id_tag: None,
    }
  }

  pub fn serial_number(mut self, id: impl Into<String>) -> Self {
    self.serial_number = Some(id.into());
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

  pub fn id_tag(mut self, id_tag: impl Into<String>) -> Self {
    self.id_tag = Some(id_tag.into());
    self
  }

  pub fn build(self) -> MessageGeneratorConfig {
    let config_default = MessageGeneratorConfig::default();

    MessageGeneratorConfig {
      serial_number: self.serial_number.unwrap_or(config_default.serial_number),
      vendor: self.vendor.unwrap_or(config_default.vendor),
      model: self.model.unwrap_or(config_default.model),
      id_tag: self.id_tag.unwrap_or(config_default.id_tag),
    }
  }
}
