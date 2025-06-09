use std::sync::atomic::{AtomicUsize, Ordering};

use chrono::Utc;
use rust_ocpp::v2_0_1::datatypes::transaction_type::TransactionType;
use rust_ocpp::v2_0_1::enumerations::connector_status_enum_type::ConnectorStatusEnumType;
use rust_ocpp::v2_0_1::enumerations::firmware_status_enum_type::FirmwareStatusEnumType;
use rust_ocpp::v2_0_1::enumerations::id_token_enum_type::IdTokenEnumType;
use rust_ocpp::v2_0_1::enumerations::transaction_event_enum_type::TransactionEventEnumType;
use rust_ocpp::v2_0_1::enumerations::trigger_reason_enum_type::TriggerReasonEnumType;
use rust_ocpp::v2_0_1::messages::transaction_event::TransactionEventRequest;
use rust_ocpp::v2_0_1::messages::{
  authorize::AuthorizeRequest, boot_notification::BootNotificationRequest,
  datatransfer::DataTransferRequest,
  heartbeat::HeartbeatRequest,
  firmware_status_notification::FirmwareStatusNotificationRequest,
  meter_values::MeterValuesRequest,
  status_notification::StatusNotificationRequest
};

use rust_ocpp::v2_0_1::enumerations::boot_reason_enum_type::BootReasonEnumType;
use rust_ocpp::v2_0_1::datatypes::charging_station_type::ChargingStationType;
use rust_ocpp::v2_0_1::datatypes::id_token_type::IdTokenType;

use serde::Serialize;
use serde_json::{Value, json};

use crate::message::{MessageGeneratorConfig, MessageGeneratorTrait};
use crate::ocpp::OcppActionType;
use uuid::Uuid;

use super::types::OcppAction;

pub struct MessageGenerator {
  config: MessageGeneratorConfig,
  id_counter: AtomicUsize,
}

struct FrameBuilder {
  ocpp_action: OcppAction,
  payload: Value
}

impl FrameBuilder {
  fn new<T: Serialize>(ocpp_action: OcppAction, payload: T) -> Value {
    // let id = self.next_id();
    json!([2, Uuid::new_v4(), ocpp_action, payload])
  }
}

impl MessageGeneratorTrait for MessageGenerator {
  fn boot_notification(&self) -> Value {
    FrameBuilder::new(OcppAction::BootNotification, BootNotificationRequest {
      reason: BootReasonEnumType::PowerUp,
      charging_station: ChargingStationType {
        model: self.config.model.clone(),
        vendor_name: self.config.vendor.clone(),
        firmware_version: Some("1.2.3".to_string()),
        ..Default::default()
      },
      ..Default::default()
    })
  }

  fn heartbeat(&self) -> Value {
    FrameBuilder::new(OcppAction::Heartbeat, HeartbeatRequest {})
  }

  fn authorize(&self) -> Value {
    FrameBuilder::new(OcppAction::Authorize, AuthorizeRequest {
      id_token: IdTokenType {
        id_token: self.config.id_tag.clone(),
        additional_info: None,
        kind: IdTokenEnumType::Central
      },
      ..Default::default()
    })
  }
  //
  // fn start_transaction(&self) -> Self::StartTransaction {
  //   TransactionEventRequest {
  //     event_type: TransactionEventEnumType::Started,
  //     timestamp: chrono::Utc::now(),
  //     trigger_reason: TriggerReasonEnumType::CablePluggedIn,
  //     seq_no: 1,
  //     transaction_info: TransactionType {
  //       transaction_id: "42".to_string(),
  //       ..Default::default()
  //     },
  //     ..Default::default()
  //   }
  // }
  //
  // fn stop_transaction(&self) -> Self::StopTransaction {
  //   TransactionEventRequest {
  //     event_type: TransactionEventEnumType::Ended,
  //     timestamp: chrono::Utc::now(),
  //     trigger_reason: TriggerReasonEnumType::RemoteStop,
  //     seq_no: 1,
  //     transaction_info: TransactionType {
  //       transaction_id: "42".to_string(),
  //       ..Default::default()
  //     },
  //     ..Default::default()
  //   }
  // }
  //
  // fn status_notification(&self) -> Self::StatusNotification {
  //   StatusNotificationRequest {
  //     timestamp: Utc::now(),
  //     evse_id: 1,
  //     connector_id: 1,
  //     connector_status: ConnectorStatusEnumType::Available,
  //     ..Default::default()
  //   }
  // }
  //
  // fn meter_values(&self) -> Self::MeterValues {
  //   MeterValuesRequest {
  //     evse_id: 1,
  //     meter_value: Default::default(),
  //   }
  // }
  //
  // fn diagnostics_status_notification(&self) -> Self::DiagnosticsStatusNotification {
  //   HeartbeatRequest {}
  // }
  //
  // fn firmware_status_notification(&self) -> Self::FirmwareStatusNotification {
  //   FirmwareStatusNotificationRequest {
  //     status: FirmwareStatusEnumType::Installed,
  //     ..Default::default()
  //   }
  // }
  //
  // fn data_transfer(&self) -> Self::DataTransfer {
  //   DataTransferRequest {
  //     vendor_id: self.config.vendor.clone(),
  //     data: Some("test".to_string()),
  //     ..Default::default()
  //   }
  // }

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
