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

use crate::message_generator::MessageGenerator;
use uuid::Uuid;

use super::types::OcppAction;

pub struct Generator {}

impl MessageGenerator for Generator {
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

  fn boot_notification() -> Self::BootNotification {
    BootNotificationRequest {
      charge_point_model: "TEST".into(),
      charge_point_vendor: "TEST".into(),
      ..Default::default()
    }
  }

  fn heartbeat() -> Self::Heartbeat {
    HeartbeatRequest {}
  }

  fn authorize() -> Self::Authorize {
    AuthorizeRequest {
      id_tag: "TEST".into(),
    }
  }

  fn start_transaction() -> Self::StartTransaction {
    StartTransactionRequest {
      connector_id: 1,
      id_tag: "TEST".into(),
      meter_start: 0,
      timestamp: chrono::Utc::now(),
      ..Default::default()
    }
  }

  fn stop_transaction() -> Self::StopTransaction {
    StopTransactionRequest {
      meter_stop: 10,
      timestamp: chrono::Utc::now(),
      id_tag: Some("TEST".into()),
      transaction_id: 1,
      ..Default::default()
    }
  }

  fn status_notification() -> Self::StatusNotification {
    StatusNotificationRequest {
      connector_id: 1,
      error_code: ChargePointErrorCode::NoError,
      status: ChargePointStatus::Available,
      timestamp: Some(chrono::Utc::now()),
      ..Default::default()
    }
  }

  fn meter_values() -> Self::MeterValues {
    MeterValuesRequest {
      connector_id: 1,
      meter_value: vec![],
      transaction_id: Some(1),
    }
  }

  fn diagnostics_status_notification() -> Self::DiagnosticsStatusNotification {
    DiagnosticsStatusNotificationRequest {
      status: DiagnosticsStatus::Uploaded,
    }
  }

  fn firmware_status_notification() -> Self::FirmwareStatusNotification {
    FirmwareStatusNotificationRequest {
      status: FirmwareStatus::Installed,
    }
  }

  fn data_transfer() -> Self::DataTransfer {
    DataTransferRequest {
      vendor_string: "TEST".into(),
      ..Default::default()
    }
  }

  fn to_frame<T: Serialize>(action: Self::OcppAction, payload: T) -> Value {
    json!([2, Uuid::new_v4().to_string(), action, payload])
  }
}
