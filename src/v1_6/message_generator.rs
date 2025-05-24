use rust_ocpp::v1_6::messages::{
  authorize::AuthorizeRequest, boot_notification::BootNotificationRequest,
  data_transfer::DataTransferRequest,
  diagnostics_status_notification::DiagnosticsStatusNotificationRequest,
  firmware_status_notification::FirmwareStatusNotificationRequest, heart_beat::HeartbeatRequest,
  meter_values::MeterValuesRequest, start_transaction::StartTransactionRequest,
  status_notification::StatusNotificationRequest, stop_transaction::StopTransactionRequest,
};

use rust_ocpp::v1_6::types::ChargePointErrorCode;
use rust_ocpp::v1_6::types::ChargePointStatus;
use rust_ocpp::v1_6::types::DiagnosticsStatus;
use rust_ocpp::v1_6::types::FirmwareStatus;
use serde::Serialize;
use serde_json::{Value, json};

use crate::message_generator::MessageGeneratorTrait;
use uuid::Uuid;

use super::types::OcppAction;

pub struct MessageGeneratorConfig {
  serial_number: String,
  vendor: String,
  model: String,
  id_tag: String
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
      id_tag: None
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


pub struct MessageGenerator {
  config: MessageGeneratorConfig
}

impl MessageGeneratorTrait for MessageGenerator {
  type OcppAction = OcppAction;

  type BootNotification = BootNotificationRequest;
  type Heartbeat = HeartbeatRequest;
  type Authorize = AuthorizeRequest;
  type StartTransaction = StartTransactionRequest;
  type StopTransaction = StopTransactionRequest;
  type StatusNotification = StatusNotificationRequest;
  type MeterValues = MeterValuesRequest;
  type DiagnosticsStatusNotification = DiagnosticsStatusNotificationRequest;
  type FirmwareStatusNotification = FirmwareStatusNotificationRequest;
  type DataTransfer = DataTransferRequest;

  fn boot_notification(&self) -> Self::BootNotification {
    BootNotificationRequest {
      charge_point_model: self.config.model.clone(),
      charge_point_vendor: self.config.vendor.clone(),
      ..Default::default()
    }
  }

  fn heartbeat(&self) -> Self::Heartbeat {
    HeartbeatRequest {}
  }

  fn authorize(&self) -> Self::Authorize {
    AuthorizeRequest {
      id_tag: self.config.id_tag.clone(),
    }
  }

  fn start_transaction(&self) -> Self::StartTransaction {
    StartTransactionRequest {
      connector_id: 1,
      id_tag: self.config.id_tag.clone(),
      meter_start: 0,
      timestamp: chrono::Utc::now(),
      ..Default::default()
    }
  }

  fn stop_transaction(&self) -> Self::StopTransaction {
    StopTransactionRequest {
      meter_stop: 10,
      timestamp: chrono::Utc::now(),
      id_tag: Some(self.config.id_tag.clone()),
      transaction_id: 1,
      ..Default::default()
    }
  }

  fn status_notification(&self) -> Self::StatusNotification {
    StatusNotificationRequest {
      connector_id: 1,
      error_code: ChargePointErrorCode::NoError,
      status: ChargePointStatus::Available,
      timestamp: Some(chrono::Utc::now()),
      ..Default::default()
    }
  }

  fn meter_values(&self) -> Self::MeterValues {
    MeterValuesRequest {
      connector_id: 1,
      meter_value: vec![],
      transaction_id: Some(1),
    }
  }

  fn diagnostics_status_notification(&self) -> Self::DiagnosticsStatusNotification {
    DiagnosticsStatusNotificationRequest {
      status: DiagnosticsStatus::Uploaded,
    }
  }

  fn firmware_status_notification(&self) -> Self::FirmwareStatusNotification {
    FirmwareStatusNotificationRequest {
      status: FirmwareStatus::Installed,
    }
  }

  fn data_transfer(&self) -> Self::DataTransfer {
    DataTransferRequest {
      vendor_string: self.config.vendor.clone(),
      ..Default::default()
    }
  }

  fn to_frame<T: Serialize>(action: Self::OcppAction, payload: T) -> Value {
    json!([2, Uuid::new_v4().to_string(), action, payload])
  }
}

impl MessageGenerator {
  pub fn new(config: MessageGeneratorConfig) -> Self {
    Self {
      config
    }
  }
}
