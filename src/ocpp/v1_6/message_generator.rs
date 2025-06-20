use std::sync::atomic::{AtomicUsize, Ordering};

use rust_ocpp::v1_6::messages::heart_beat::HeartbeatResponse;
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

use uuid::Uuid;

use crate::ocpp::message_generator::{
  MessageBuilderTrait, MessageGeneratorConfig, MessageGeneratorTrait,
};

use super::types::OcppAction;

pub struct MessageGenerator {
  config: MessageGeneratorConfig,
  id_counter: AtomicUsize,
}

struct FrameBuilder {
  ocpp_action: OcppAction,
  payload: Value,
}

impl FrameBuilder {
  fn build_call<T: Serialize>(ocpp_action: OcppAction, payload: T) -> Value {
    // let id = self.next_id();
    json!([2, Uuid::new_v4(), ocpp_action, payload])
  }

  pub fn build_call_result<T: Serialize>(message_id: &str, payload: T) -> Value {
    json!([3, message_id, payload])
  }

  pub fn build_call_error(
    message_id: &str,
    error_code: &str,
    error_description: &str,
    error_details: Option<Value>,
  ) -> Value {
    json!([
      4,
      message_id,
      error_code,
      error_description,
      error_details.unwrap_or_else(|| json!({}))
    ])
  }
}

impl MessageGeneratorTrait for MessageGenerator {
  // Charger -> CSMS

  fn boot_notification(&self) -> Value {
    FrameBuilder::build_call(
      OcppAction::BootNotification,
      BootNotificationRequest {
        charge_point_model: self.config.model.clone(),
        charge_point_vendor: self.config.vendor.clone(),
        ..Default::default()
      },
    )
  }

  fn heartbeat(&self) -> Value {
    FrameBuilder::build_call(OcppAction::Heartbeat, HeartbeatRequest {})
  }

  fn authorize(&self) -> Value {
    FrameBuilder::build_call(
      OcppAction::Authorize,
      AuthorizeRequest {
        id_tag: self.config.id_tag.clone(),
      },
    )
  }

  fn start_transaction(&self) -> Value {
    FrameBuilder::build_call(
      OcppAction::StartTransaction,
      StartTransactionRequest {
        connector_id: 1,
        id_tag: self.config.id_tag.clone(),
        meter_start: 0,
        timestamp: chrono::Utc::now(),
        ..Default::default()
      },
    )
  }

  fn stop_transaction(&self) -> Value {
    FrameBuilder::build_call(
      OcppAction::StopTransaction,
      StopTransactionRequest {
        meter_stop: 10,
        timestamp: chrono::Utc::now(),
        id_tag: Some(self.config.id_tag.clone()),
        transaction_id: 1,
        ..Default::default()
      },
    )
  }

  fn status_notification(&self) -> Value {
    FrameBuilder::build_call(
      OcppAction::StatusNotification,
      StatusNotificationRequest {
        connector_id: 1,
        error_code: ChargePointErrorCode::NoError,
        status: ChargePointStatus::Available,
        timestamp: Some(chrono::Utc::now()),
        ..Default::default()
      },
    )
  }

  fn meter_values(&self) -> Value {
    FrameBuilder::build_call(
      OcppAction::MeterValues,
      MeterValuesRequest {
        connector_id: 1,
        meter_value: vec![],
        transaction_id: Some(1),
      },
    )
  }

  fn diagnostics_status_notification(&self) -> Value {
    FrameBuilder::build_call(
      OcppAction::DiagnosticsStatusNotification,
      DiagnosticsStatusNotificationRequest {
        status: DiagnosticsStatus::Uploaded,
      },
    )
  }

  fn firmware_status_notification(&self) -> Value {
    FrameBuilder::build_call(
      OcppAction::FirmwareStatusNotification,
      FirmwareStatusNotificationRequest {
        status: FirmwareStatus::Installed,
      },
    )
  }

  fn data_transfer(&self) -> Value {
    FrameBuilder::build_call(
      OcppAction::StatusNotification,
      DataTransferRequest {
        vendor_string: self.config.vendor.clone(),
        ..Default::default()
      },
    )
  }

  fn next_id(&self) -> String {
    self.id_counter.fetch_add(1, Ordering::Relaxed).to_string()
  }
}

impl MessageGenerator {
  pub fn new(config: MessageGeneratorConfig) -> Self {
    Self {
      config,
      id_counter: AtomicUsize::new(1),
    }
  }
}
