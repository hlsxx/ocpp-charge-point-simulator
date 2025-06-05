use serde::Serialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::ocpp::OcppActionType;

pub trait MessageBuilderTrait {
  fn to_call_frame(&self) -> Value;
}

pub trait MessageGeneratorTrait: Send {
  fn boot_notification(&self) -> Value;
  fn heartbeat(&self) -> Value;
  fn authorize(&self) -> Value;
  // fn start_transaction(&self) -> Value;
  // fn stop_transaction(&self) -> Value;
  // fn status_notification(&self) -> Value;
  // fn meter_values(&self) -> Value;
  // fn diagnostics_status_notification(&self) -> Value;
  // fn firmware_status_notification(&self) -> Value;
  // fn data_transfer(&self) -> Value;

  fn next_id(&self) -> String;
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
