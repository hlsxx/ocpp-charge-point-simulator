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

use crate::message::{MessageBuilderTrait, MessageGeneratorConfig, MessageGeneratorTrait};
use crate::ocpp::OcppActionType;
use uuid::Uuid;

use super::types::OcppAction;

pub struct MessageGenerator {
  config: MessageGeneratorConfig,
  id_counter: AtomicUsize,
}

struct MessageBuilder {
  ocpp_action: OcppAction,
  payload: Value
}

impl MessageBuilder {
  fn new(ocpp_action: OcppAction, payload: Value) -> Self {
    Self {
      ocpp_action,
      payload
    }
  }
}

impl MessageBuilderTrait for MessageBuilder {
  fn to_call_frame(&self) -> Value {
    let id = self.next_id();
    json!([2, id, self.ocpp_action, self.payload])
  }
}

impl MessageGeneratorTrait for MessageGenerator {
  fn boot_notification(&self) -> Value {
    MessageBuilder::new(OcppAction::BootNotification, BootNotificationRequest {
      charge_point_model: self.config.model.clone(),
      charge_point_vendor: self.config.vendor.clone(),
      ..Default::default()
    }).to_call_frame()
  }

  fn heartbeat(&self) -> Value {
    MessageBuilder::new(OcppAction::Heartbeat, HeartbeatRequest {}).to_call_frame()
  }

  fn authorize(&self) -> Value {
    MessageBuilder::new(OcppAction::Authorize,
      AuthorizeRequest {
        id_tag: self.config.id_tag.clone(),
      }
    ).to_call_frame()
  }

  // fn start_transaction(&self) -> Value {
  //   MessageBuilder::new(OcppAction::StartTransaction, StartTransactionRequest {
  //     connector_id: 1,
  //     id_tag: self.config.id_tag.clone(),
  //     meter_start: 0,
  //     timestamp: chrono::Utc::now(),
  //     ..Default::default()
  //   }).to_call_frame()
  // }
  //
  // fn stop_transaction(&self) -> Value {
  //   MessageBuilder::new(OcppAction::StopTransaction, StopTransactionRequest {
  //     meter_stop: 10,
  //     timestamp: chrono::Utc::now(),
  //     id_tag: Some(self.config.id_tag.clone()),
  //     transaction_id: 1,
  //     ..Default::default()
  //   }).to_call_frame()
  // }

  // fn status_notification(&self) -> Self::StatusNotification {
  //   StatusNotificationRequest {
  //     connector_id: 1,
  //     error_code: ChargePointErrorCode::NoError,
  //     status: ChargePointStatus::Available,
  //     timestamp: Some(chrono::Utc::now()),
  //     ..Default::default()
  //   }
  // }

  // fn meter_values(&self) -> Self::MeterValues {
  //   MeterValuesRequest {
  //     connector_id: 1,
  //     meter_value: vec![],
  //     transaction_id: Some(1),
  //   }
  // }

  // fn diagnostics_status_notification(&self) -> Self::DiagnosticsStatusNotification {
  //   DiagnosticsStatusNotificationRequest {
  //     status: DiagnosticsStatus::Uploaded,
  //   }
  // }
  //
  // fn firmware_status_notification(&self) -> Self::FirmwareStatusNotification {
  //   FirmwareStatusNotificationRequest {
  //     status: FirmwareStatus::Installed,
  //   }
  // }
  //
  // fn data_transfer(&self) -> Self::DataTransfer {
  //   DataTransferRequest {
  //     vendor_string: self.config.vendor.clone(),
  //     ..Default::default()
  //   }
  // }
  //
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
