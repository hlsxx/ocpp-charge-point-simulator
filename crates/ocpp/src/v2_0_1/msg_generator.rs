use std::sync::atomic::{AtomicUsize, Ordering};

use async_trait::async_trait;
use chrono::Utc;
use rust_ocpp::v2_0_1::datatypes::transaction_type::TransactionType;
use rust_ocpp::v2_0_1::enumerations::firmware_status_enum_type::FirmwareStatusEnumType;
use rust_ocpp::v2_0_1::enumerations::id_token_enum_type::IdTokenEnumType;
use rust_ocpp::v2_0_1::enumerations::transaction_event_enum_type::TransactionEventEnumType;
use rust_ocpp::v2_0_1::enumerations::trigger_reason_enum_type::TriggerReasonEnumType;
use rust_ocpp::v2_0_1::messages::transaction_event::TransactionEventRequest;
use rust_ocpp::v2_0_1::messages::{
  authorize::AuthorizeRequest, boot_notification::BootNotificationRequest,
  datatransfer::DataTransferRequest,
  firmware_status_notification::FirmwareStatusNotificationRequest, heartbeat::HeartbeatRequest,
  meter_values::MeterValuesRequest, status_notification::StatusNotificationRequest,
};

use rust_ocpp::v2_0_1::datatypes::charging_station_type::ChargingStationType;
use rust_ocpp::v2_0_1::datatypes::id_token_type::IdTokenType;
use rust_ocpp::v2_0_1::enumerations::boot_reason_enum_type::BootReasonEnumType;

use serde::Serialize;
use serde_json::{Value, json};

use super::types::OcppAction;
use crate::msg_generator::{MessageGenerator, MessageGeneratorConfig};
use crate::types::CommonConnectorStatusType;
use uuid::Uuid;

pub struct V201MessageGenerator {
  config: MessageGeneratorConfig,
  id_counter: AtomicUsize,
}

struct FrameBuilder;

impl FrameBuilder {
  pub fn build_call<T: Serialize>(action: impl ToString, payload: T) -> Value {
    json!({
      "messageTypeId": 2,
      "messageId": Uuid::new_v4().to_string(),
      "action": action.to_string(),
      "payload": payload
    })
  }

  #[allow(unused)]
  pub fn build_call_result<T: Serialize>(message_id: &str, payload: T) -> Value {
    json!({
      "messageTypeId": 3,
      "messageId": message_id,
      "payload": payload
    })
  }

  #[allow(unused)]
  pub fn build_call_error(
    message_id: &str,
    error_code: &str,
    error_description: &str,
    error_details: Option<Value>,
  ) -> Value {
    json!({
      "messageTypeId": 4,
      "messageId": message_id,
      "errorCode": error_code,
      "errorDescription": error_description,
      "errorDetails": error_details.unwrap_or_else(|| json!({}))
    })
  }
}

#[async_trait]
impl MessageGenerator for V201MessageGenerator {
  // Charger -> CSMS
  async fn boot_notification(&self) -> Value {
    FrameBuilder::build_call(
      OcppAction::BootNotification,
      BootNotificationRequest {
        reason: BootReasonEnumType::PowerUp,
        charging_station: ChargingStationType {
          model: self.config.model.clone(),
          vendor_name: self.config.vendor.clone(),
          firmware_version: Some("1.2.3".to_string()),
          ..Default::default()
        },
      },
    )
  }

  async fn heartbeat(&self) -> Value {
    FrameBuilder::build_call(OcppAction::Heartbeat, HeartbeatRequest {})
  }

  async fn authorize(&self) -> Value {
    FrameBuilder::build_call(
      OcppAction::Authorize,
      AuthorizeRequest {
        id_token: IdTokenType {
          id_token: self.config.id_tag.clone(),
          additional_info: None,
          kind: IdTokenEnumType::Central,
        },
        ..Default::default()
      },
    )
  }

  async fn start_transaction(&self, tag_id: Option<&str>) -> Value {
    FrameBuilder::build_call(
      OcppAction::TransactionEvent,
      TransactionEventRequest {
        event_type: TransactionEventEnumType::Started,
        timestamp: chrono::Utc::now(),
        trigger_reason: TriggerReasonEnumType::CablePluggedIn,
        seq_no: 1,
        transaction_info: TransactionType {
          transaction_id: "42".to_string(),
          ..Default::default()
        },
        ..Default::default()
      },
    )
  }

  async fn stop_transaction(&self) -> Value {
    FrameBuilder::build_call(
      OcppAction::TransactionEvent,
      TransactionEventRequest {
        event_type: TransactionEventEnumType::Ended,
        timestamp: chrono::Utc::now(),
        trigger_reason: TriggerReasonEnumType::RemoteStop,
        seq_no: 1,
        transaction_info: TransactionType {
          transaction_id: "42".to_string(),
          ..Default::default()
        },
        ..Default::default()
      },
    )
  }

  async fn status_notification(&self, status: CommonConnectorStatusType) -> Value {
    FrameBuilder::build_call(
      OcppAction::StatusNotification,
      StatusNotificationRequest {
        timestamp: Utc::now(),
        evse_id: 1,
        connector_id: 1,
        connector_status: status.into(),
      },
    )
  }

  async fn meter_values(&self) -> Value {
    FrameBuilder::build_call(
      OcppAction::MeterValues,
      MeterValuesRequest {
        evse_id: 1,
        meter_value: Default::default(),
      },
    )
  }

  async fn diagnostics_status_notification(&self) -> Value {
    FrameBuilder::build_call(OcppAction::Heartbeat, HeartbeatRequest {})
  }

  async fn firmware_status_notification(&self) -> Value {
    FrameBuilder::build_call(
      OcppAction::FirmwareStatusNotification,
      FirmwareStatusNotificationRequest {
        status: FirmwareStatusEnumType::Installed,
        ..Default::default()
      },
    )
  }

  async fn data_transfer(&self) -> Value {
    FrameBuilder::build_call(
      OcppAction::DataTransfer,
      DataTransferRequest {
        vendor_id: self.config.vendor.clone(),
        data: Some("test".to_string()),
        ..Default::default()
      },
    )
  }

  fn next_id(&self) -> String {
    self.id_counter.fetch_add(1, Ordering::Relaxed).to_string()
  }
}

impl V201MessageGenerator {
  pub fn new(config: MessageGeneratorConfig) -> Self {
    Self {
      config,
      id_counter: AtomicUsize::new(1),
    }
  }
}
