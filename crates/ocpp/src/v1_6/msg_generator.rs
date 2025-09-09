use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};

use async_trait::async_trait;
use common::SharedData;
use rust_ocpp::v1_6::messages::{
  authorize::AuthorizeRequest, boot_notification::BootNotificationRequest,
  data_transfer::DataTransferRequest,
  diagnostics_status_notification::DiagnosticsStatusNotificationRequest,
  firmware_status_notification::FirmwareStatusNotificationRequest, heart_beat::HeartbeatRequest,
  meter_values::MeterValuesRequest, start_transaction::StartTransactionRequest,
  status_notification::StatusNotificationRequest, stop_transaction::StopTransactionRequest,
};

use rust_ocpp::v1_6::types::DiagnosticsStatus;
use rust_ocpp::v1_6::types::FirmwareStatus;
use rust_ocpp::v1_6::types::{ChargePointErrorCode, MeterValue};
use serde::Serialize;
use serde_json::{Value, json};

use tracing::{debug, info};
use uuid::Uuid;

use crate::mock_data::MockData;
use crate::msg_generator::{MessageGeneratorConfig, MessageGeneratorTrait};
use crate::types::CommonConnectorStatusType;

use super::types::OcppAction;

struct FrameBuilder;

impl FrameBuilder {
  async fn build_call<T>(
    shared_data: &SharedData<OcppAction>,
    ocpp_action: OcppAction,
    payload: T,
  ) -> Value
  where
    T: Debug + Serialize,
  {
    info!("🔌 [🔵 Call] {}", ocpp_action);
    debug!(action = %ocpp_action, ?payload);

    let msg_id = Uuid::new_v4();
    shared_data
      .insert_msg(&msg_id.to_string(), ocpp_action.clone())
      .await;

    json!([2, msg_id, ocpp_action, payload])
  }

  #[allow(unused)]
  pub fn build_call_result<T: Serialize>(message_id: &str, payload: T) -> Value {
    json!([3, message_id, payload])
  }

  #[allow(unused)]
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

pub struct MessageGenerator {
  config: MessageGeneratorConfig,
  shared_data: SharedData<OcppAction>,
  id_counter: AtomicUsize,
}

#[async_trait]
impl MessageGeneratorTrait for MessageGenerator {
  type StatusType = CommonConnectorStatusType;

  async fn boot_notification(&self) -> Value {
    self
      .build_call(
        OcppAction::BootNotification,
        BootNotificationRequest {
          charge_point_model: self.config.model.clone(),
          charge_point_vendor: self.config.vendor.clone(),
          ..Default::default()
        },
      )
      .await
  }

  async fn heartbeat(&self) -> Value {
    self
      .build_call(OcppAction::Heartbeat, HeartbeatRequest {})
      .await
  }

  async fn authorize(&self) -> Value {
    self
      .build_call(
        OcppAction::Authorize,
        AuthorizeRequest {
          id_tag: self.config.id_tag.clone(),
        },
      )
      .await
  }

  async fn start_transaction(&self) -> Value {
    self
      .build_call(
        OcppAction::StartTransaction,
        StartTransactionRequest {
          connector_id: 1,
          id_tag: self.config.id_tag.clone(),
          meter_start: 0,
          timestamp: chrono::Utc::now(),
          ..Default::default()
        },
      )
      .await
  }

  async fn stop_transaction(&self) -> Value {
    self
      .build_call(
        OcppAction::StopTransaction,
        StopTransactionRequest {
          meter_stop: 10,
          timestamp: chrono::Utc::now(),
          id_tag: Some(self.config.id_tag.clone()),
          transaction_id: self.shared_data.get_transaction_id().await.unwrap_or(1),
          ..Default::default()
        },
      )
      .await
  }

  async fn status_notification(&self, status: Self::StatusType) -> Value {
    self
      .build_call(
        OcppAction::StatusNotification,
        StatusNotificationRequest {
          connector_id: 1,
          error_code: ChargePointErrorCode::NoError,
          status: status.into(),
          timestamp: Some(chrono::Utc::now()),
          ..Default::default()
        },
      )
      .await
  }

  async fn meter_values(&self) -> Value {
    let transaction_id = self.shared_data.get_transaction_id().await;

    if let Some(transaction_id) = transaction_id {
      self
        .build_call(
          OcppAction::MeterValues,
          MeterValuesRequest {
            connector_id: 1,
            meter_value: vec![MeterValue::mock_data()],
            transaction_id: Some(transaction_id),
          },
        )
        .await
    } else {
      Value::Null
    }
  }

  async fn diagnostics_status_notification(&self) -> Value {
    self
      .build_call(
        OcppAction::DiagnosticsStatusNotification,
        DiagnosticsStatusNotificationRequest {
          status: DiagnosticsStatus::Uploaded,
        },
      )
      .await
  }

  async fn firmware_status_notification(&self) -> Value {
    self
      .build_call(
        OcppAction::FirmwareStatusNotification,
        FirmwareStatusNotificationRequest {
          status: FirmwareStatus::Installed,
        },
      )
      .await
  }

  async fn data_transfer(&self) -> Value {
    self
      .build_call(
        OcppAction::StatusNotification,
        DataTransferRequest {
          vendor_string: self.config.vendor.clone(),
          ..Default::default()
        },
      )
      .await
  }

  fn next_id(&self) -> String {
    self.id_counter.fetch_add(1, Ordering::Relaxed).to_string()
  }
}

impl MessageGenerator {
  pub fn new(config: MessageGeneratorConfig, shared_data: SharedData<OcppAction>) -> Self {
    Self {
      config,
      shared_data,
      id_counter: AtomicUsize::new(1),
    }
  }

  async fn build_call<T>(&self, ocpp_action: OcppAction, payload: T) -> Value
  where
    T: Debug + Serialize,
  {
    FrameBuilder::build_call(&self.shared_data, ocpp_action, payload).await
  }
}
