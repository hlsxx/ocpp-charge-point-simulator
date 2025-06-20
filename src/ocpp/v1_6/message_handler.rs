use std::cell::RefCell;
use std::fmt::Display;
use std::sync::atomic::Ordering;
use std::{
  fmt::Debug,
  str::FromStr,
  sync::{Arc, atomic::AtomicI32},
};

use crate::ocpp::message_handler::{ChargePointData, OcppMessageHandler, OcppMessageFrameType};
use super::types::{OcppMessageFrame, OcppAction};
use crate::models::charge_point_state::{ChargePointState as ChargePointStateModel};
use crate::{
  database::Database,
  models::{
    Model,
    tag::Tag as TagModel,
    transaction::Transaction as TransactionModel,
    meter_value::MeterValue as MeterValueModel,
    meter_sampled_value::MeterSampledValue as MeterSampledValueModel,
    charge_point::{ChargePoint as ChargePointModel, OcppVersionType},
  },
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::Utc;
use rust_ocpp::v1_6::{
  messages::{
    authorize::{AuthorizeRequest, AuthorizeResponse},
    boot_notification::{BootNotificationRequest, BootNotificationResponse},
    data_transfer::{DataTransferRequest, DataTransferResponse},
    diagnostics_status_notification::{
      DiagnosticsStatusNotificationRequest, DiagnosticsStatusNotificationResponse,
    },
    firmware_status_notification::{
      FirmwareStatusNotificationRequest, FirmwareStatusNotificationResponse,
    },
    heart_beat::{HeartbeatRequest, HeartbeatResponse},
    meter_values::{MeterValuesRequest, MeterValuesResponse},
    start_transaction::{StartTransactionRequest, StartTransactionResponse},
    status_notification::{StatusNotificationRequest, StatusNotificationResponse},
    stop_transaction::{StopTransactionRequest, StopTransactionResponse},
  },
  types::{AuthorizationStatus, DataTransferStatus, IdTagInfo, MeterValue, RegistrationStatus},
};

use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use sqlx::{PgPool, query_as};
use tracing::{debug, error, info, span, warn, Level, Span};
use super::types::{DisplayMeasurand, DisplayUnitOfMeasure, DisplayReadingContext, DisplayPhase, DisplayLocation, DisplayValueFormat};

pub struct MessageHandler {
  pub db: Arc<Database>,
  charge_point_data: Option<ChargePointData>,
}

impl MessageHandler {
  pub fn new(db: Arc<Database>) -> Self {
    Self {
      db,
      charge_point_data: None,
    }
  }
}

#[async_trait]
impl OcppMessageHandler for MessageHandler {
  fn pool(&self) -> &PgPool {
    self.db.pool()
  }

  fn charge_point_data(&self) -> Option<&ChargePointData> {
    self.charge_point_data.as_ref()
  }

  // fn transaction_id(&self) -> Option<i32> {
  //   self.charge_point_state()
  //     .map(|charge_point_state| charge_point_state.last_transaction_id)
  // }

  // fn charge_point(&self) -> Option<&ChargePointModel> {
  //   self.charge_point.as_ref()
  // }

  fn parse_ocpp_message(&self, text: &str) -> Result<OcppMessageFrameType> {
    let arr: Vec<Value> = serde_json::from_str(&text)?;

    match arr.get(0).and_then(|v| v.as_u64()) {
      Some(2) => {
        let msg_id = arr[1].as_str().unwrap_or("").to_string();
        let action_string = arr[2].as_str().unwrap_or("").to_string();
        let payload = arr[3].clone();

        let action = OcppAction::from_str(action_string.as_str())
          .map_err(|err| anyhow::anyhow!("Invalid OCPP action: {}", err))?;

        Ok(OcppMessageFrameType::V1_6(OcppMessageFrame::Call {
          msg_id,
          action,
          payload,
        }))
      }
      Some(3) => {
        let msg_id = arr[1].as_str().unwrap_or("").to_string();
        let payload = arr[2].clone();

        Ok(OcppMessageFrameType::V1_6(OcppMessageFrame::CallResult {
          msg_id,
          payload,
        }))
      }
      Some(4) => {
        let msg_id = arr[1].as_str().unwrap_or("").to_string();
        let error_code = arr[2].as_str().unwrap_or("").to_string();
        let description = arr[3].as_str().unwrap_or("").to_string();

        Ok(OcppMessageFrameType::V1_6(OcppMessageFrame::CallError {
          msg_id,
          error_code,
          description,
        }))
      }
      _ => anyhow::bail!("Unknown OCPP v1.6 message type"),
    }
  }

  async fn handle_text_message(&mut self, text: &str) -> Result<Option<String>> {
    if let OcppMessageFrameType::V1_6(ocpp_message) = self.parse_ocpp_message(text)? {
      match ocpp_message {
        OcppMessageFrame::Call {
          msg_id,
          action,
          payload,
        } => {
          info!("🔌 [🔵 Call] {}", action);
          debug!(?action, msg_id, ?payload);
          return self.handle_call(&msg_id, &action, &payload).await;
        }
        OcppMessageFrame::CallResult { msg_id, payload } => {
          info!("🔌 [🟢 CallResult]");
          debug!(msg_id, ?payload);
          return self.handle_call_result(&msg_id, &payload).await;
        }
        OcppMessageFrame::CallError {
          msg_id,
          error_code,
          description,
        } => {
          info!("🔌 [🔴 CallError] {}", error_code);
          debug!(msg_id, error_code, description);
          return self.handle_call_error(&msg_id).await;
        }
      }
    }

    anyhow::bail!("Invalid text message")
  }
}

impl MessageHandler {
  async fn handle_ocpp_request<Req, Res, F, Fut>(
    msg_id: &str,
    payload: Value,
    make_response: F,
  ) -> Result<Option<String>>
  where
    Req: DeserializeOwned,
    Res: Serialize + Debug,
    F: FnOnce(Req) -> Fut,
    Fut: Future<Output = Result<Res>>,
  {
    let request: Req = serde_json::from_value(payload)?;
    let response = make_response(request).await?;

    let ocpp_message = OcppMessageFrame::CallResult {
      msg_id: msg_id.to_string(),
      payload: serde_json::to_value(&response)?,
    };

    let response_string = serde_json::to_string(&ocpp_message.to_frame())?;

    // info!("🟥 CSMS → Charger [CALLERROR]");
    info!("🧠 [🟢 CallResult]");
    debug!(msg_id, ?response);

    Ok(Some(response_string))
  }

  async fn handle_call(
    &mut self,
    msg_id: &str,
    action: &OcppAction,
    payload: &Value,
  ) -> Result<Option<String>> {
    use OcppAction::*;

    match action {
      BootNotification => {
        Self::handle_ocpp_request::<BootNotificationRequest, BootNotificationResponse, _, _>(
          msg_id,
          payload.clone(),
          |request| async move {
            // Note: Generate UID.
            // If the UID does not exist, create a new record and return it.
            // If it already exists, return the existing record.
            let charge_point = ChargePointModel {
              uid: ChargePointModel::generate_uid(
                format!(
                  "{}{}{}{}",
                  request.charge_point_serial_number.as_deref().unwrap_or(""),
                  request.charge_box_serial_number.as_deref().unwrap_or(""),
                  request.charge_point_model,
                  request.charge_point_vendor,
                ).as_str()
              ),
              serial_number: request.charge_point_serial_number,
              model: request.charge_point_model,
              vendor: request.charge_point_vendor,
              firmware_version: request.firmware_version,
              ocpp_version: OcppVersionType::V1_6,
              ..Default::default()
            };

            let charge_point = charge_point.insert_or_get(self.pool())
              .await?;

            Span::current().record("model", charge_point.get_name());

            let charge_point_data = ChargePointData::try_new(self.pool(), charge_point)
              .await?;

            self.charge_point_data = Some(charge_point_data);

            Ok(BootNotificationResponse {
              current_time: Utc::now(),
              interval: 30,
              status: RegistrationStatus::Accepted,
            })
          },
        )
        .await
      }
      Heartbeat => {
        Self::handle_ocpp_request::<HeartbeatRequest, HeartbeatResponse, _, _>(
          msg_id,
          payload.clone(),
          |_request| async move {
            Ok(HeartbeatResponse {
              current_time: Utc::now(),
            })
          },
        )
        .await
      }
      Authorize => {
        Self::handle_ocpp_request::<AuthorizeRequest, AuthorizeResponse, _, _>(
          msg_id,
          payload.clone(),
          |request| async move {
            match TagModel::fetch_one_by_column(self.pool(), "tag_id", &request.id_tag.to_uppercase()).await {
              Ok(tag) => {
                info!("🧠 [✅ Tag {} accepted]", tag.title.as_ref().unwrap_or(&tag.tag_id));
                debug!(tag = ?tag);

                return Ok(AuthorizeResponse {
                  id_tag_info: IdTagInfo {
                    expiry_date: None,
                    parent_id_tag: None,
                    status: AuthorizationStatus::Accepted,
                  },
                });
              }
              Err(_) => {
                info!("🧠 [❌ Tag {} invalid]", request.id_tag);

                return Ok(AuthorizeResponse {
                  id_tag_info: IdTagInfo {
                    expiry_date: None,
                    parent_id_tag: None,
                    status: AuthorizationStatus::Invalid,
                  },
                });
              }
            }
          },
        )
        .await
      }
      StartTransaction => {
        Self::handle_ocpp_request::<StartTransactionRequest, StartTransactionResponse, _, _>(
          msg_id,
          payload.clone(),
          |request| async move {
            let charge_point = self.charge_point_or_err()?;

            let transaction_id = ChargePointStateModel::update_last_transaction(
              self.db.pool(),
              charge_point.id
            ).await?;

            let tag = TagModel::fetch_optional_by_column(
              self.pool(),
              "tag_id",
              &request.id_tag.to_uppercase()
            ).await.unwrap();

            match tag {
              Some(tag) => {
                info!("🧠 [✅ Tag {} accepted]", tag.title.as_ref().unwrap_or(&tag.tag_id));
                debug!(tag = ?tag);

                let transaction = TransactionModel {
                  id: 0,
                  fk_charge_point_id: charge_point.id,
                  fk_tag_id: tag.id,
                  transaction_id: transaction_id.to_string(),
                  connector_id: request.connector_id as i32,
                  started_at: request.timestamp,
                  stopped_at: None,
                  meter_start: Some(request.meter_start),
                  meter_stop: None,
                  metadata: None,
                  created_at: Utc::now(),
                  updated_at: None,
                };

                let _inserted_transaction_id = transaction.insert(self.pool())
                  .await?;

                return Ok(StartTransactionResponse {
                  transaction_id,
                  id_tag_info: IdTagInfo {
                    expiry_date: None,
                    parent_id_tag: None,
                    status: AuthorizationStatus::Accepted,
                  },
                });
              }
              None => {
                info!("🧠 [❌ Tag {} invalid]", request.id_tag);
                return Ok(StartTransactionResponse {
                  transaction_id,
                  id_tag_info: IdTagInfo {
                    expiry_date: None,
                    parent_id_tag: None,
                    status: AuthorizationStatus::Invalid,
                  },
                });
              }
            }
          },
        )
        .await
      }
      StopTransaction => {
        Self::handle_ocpp_request::<StopTransactionRequest, StopTransactionResponse, _, _>(
          msg_id,
          payload.clone(),
          |request| async move {
            println!("Stop transaction");
            let charge_point = self.charge_point_or_err()?;

            TransactionModel::set_transaction_stop(
              self.pool(),
              charge_point.id,
              request.transaction_id,
              request.timestamp,
              request.meter_stop
            ).await.unwrap();

            Ok(StopTransactionResponse { id_tag_info: None })
          },
        )
        .await
      }
      StatusNotification => {
        Self::handle_ocpp_request::<StatusNotificationRequest, StatusNotificationResponse, _, _>(
          msg_id,
          payload.clone(),
          |_request| async move { Ok(StatusNotificationResponse {}) },
        )
        .await
      }
      MeterValues => {
        Self::handle_ocpp_request::<MeterValuesRequest, MeterValuesResponse, _, _>(
          msg_id,
          payload.clone(),
          |request| async move {
            let cp_transaction_id = request.transaction_id.map(|id| id.to_string());

            let charge_point = self.charge_point_or_err()?;

            let fk_transaction_id = match cp_transaction_id {
              Some(cp_transaction_id) => {
                let transaction = TransactionModel::get_by_charge_point_and_transaction_id(
                  self.pool(),
                  charge_point.id,
                  &cp_transaction_id,
                ).await?;

                transaction.map(|record| record.id)
              },
              None => None
            };

            for meter_value in request.meter_value {
              let meter_value_model = MeterValueModel {
                id: 0,
                fk_transaction_id,
                timestamp: meter_value.timestamp,
                created_at: Utc::now(),
              };

              let inserted_meter_value_id = meter_value_model.insert(self.pool()).await.unwrap();

              for sampled_value in meter_value.sampled_value {
                let meter_sampled_value = MeterSampledValueModel {
                  id: 0,
                  fk_meter_value_id: inserted_meter_value_id,
                  measurand: sampled_value.measurand.map(|val| DisplayMeasurand(val).to_string()),
                  value: sampled_value.value.parse::<f64>().unwrap(),
                  unit: sampled_value.unit.map(|val| DisplayUnitOfMeasure(val).to_string()),
                  multiplier: None,
                  context: sampled_value.context.map(|val| DisplayReadingContext(val).to_string()),
                  location: sampled_value.location.map(|val| DisplayLocation(val).to_string()),
                  phase: sampled_value.phase.map(|val| DisplayPhase(val).to_string()),
                  format: sampled_value.format.map(|val| DisplayValueFormat(val).to_string()),
                  created_at: Utc::now()
                };

                let _ = meter_sampled_value.insert(self.pool()).await?;
              }
            }

            Ok(MeterValuesResponse {})
          },
        )
        .await
      }
      DiagnosticsStatusNotification => {
        Self::handle_ocpp_request::<
          DiagnosticsStatusNotificationRequest,
          DiagnosticsStatusNotificationResponse,
          _,
          _,
        >(msg_id, payload.clone(), |_request| async move {
          Ok(DiagnosticsStatusNotificationResponse {})
        })
        .await
      }
      FirmwareStatusNotification => {
        Self::handle_ocpp_request::<
          FirmwareStatusNotificationRequest,
          FirmwareStatusNotificationResponse,
          _,
          _,
        >(msg_id, payload.clone(), |_request| async move {
          Ok(FirmwareStatusNotificationResponse {})
        })
        .await
      }
      DataTransfer => {
        Self::handle_ocpp_request::<DataTransferRequest, DataTransferResponse, _, _>(
          msg_id,
          payload.clone(),
          |_request| async move {
            Ok(DataTransferResponse {
              status: DataTransferStatus::Accepted,
              data: None,
            })
          },
        )
        .await
      }
      _ => anyhow::bail!("Unknown OCPP action to parse payload"), // CSMS → CP
                                                                  // RemoteStartTransaction,
                                                                  // RemoteStopTransaction,
                                                                  // Reset,
                                                                  // ChangeAvailability,
                                                                  // ChangeConfiguration,
                                                                  // GetConfiguration,
                                                                  // ClearCache,
                                                                  // UpdateFirmware,
                                                                  // GetDiagnostics,
                                                                  // UnlockConnector,
                                                                  // CancelReservation,
                                                                  // ReserveNow,
                                                                  // SetChargingProfile,
                                                                  // ClearChargingProfile,
                                                                  // GetCompositeSchedule,
                                                                  // GetLocalListVersion,
                                                                  // SendLocalList,
    }
  }

  async fn handle_call_error(
    &self,
    msg_id: &str,
  ) -> Result<Option<String>> {
    use OcppAction::*;

    Ok(None)
  }

  async fn handle_call_result(
    &self,
    msg_id: &str,
    payload: &Value,
  ) -> Result<Option<String>> {
    use OcppAction::*;

    Ok(None)
    // match action {
    //   BootNotification => {
    //     Self::handle_ocpp_request::<BootNotificationRequest, BootNotificationResponse, _, _>(
    //       msg_id,
    //       payload.clone(),
    //       |_request| async move {
    //         BootNotificationResponse {
    //           current_time: Utc::now(),
    //           interval: 30,
    //           status: RegistrationStatus::Accepted,
    //         }
    //       },
    //     )
    //     .await
    //   }
    //   Heartbeat => {
    //     Self::handle_ocpp_request::<HeartbeatRequest, HeartbeatResponse, _, _>(
    //       msg_id,
    //       payload.clone(),
    //       |_request| async move {
    //         HeartbeatResponse {
    //           current_time: Utc::now(),
    //         }
    //       },
    //     )
    //     .await
    //   }
    //   Authorize => {
    //     Self::handle_ocpp_request::<AuthorizeRequest, AuthorizeResponse, _, _>(
    //       msg_id,
    //       payload.clone(),
    //       |request| async move {
    //         match Tag::get_one_by_custom(self.pool(), "tag", &request.id_tag.to_lowercase()).await {
    //           Ok(tag) => {
    //             info!(tag = request.id_tag, "tag accepted");
    //             debug!(tag = ?tag);
    //
    //             return AuthorizeResponse {
    //               id_tag_info: IdTagInfo {
    //                 expiry_date: None,
    //                 parent_id_tag: None,
    //                 status: AuthorizationStatus::Accepted,
    //               },
    //             };
    //           }
    //           Err(_) => {
    //             error!(tag = request.id_tag, "tag invalid");
    //
    //             return AuthorizeResponse {
    //               id_tag_info: IdTagInfo {
    //                 expiry_date: None,
    //                 parent_id_tag: None,
    //                 status: AuthorizationStatus::Invalid,
    //               },
    //             };
    //           }
    //         }
    //       },
    //     )
    //     .await
    //   }
    //   StartTransaction => {
    //     Self::handle_ocpp_request::<StartTransactionRequest, StartTransactionResponse, _, _>(
    //       msg_id,
    //       payload.clone(),
    //       |request| async move {
    //         let transaction_id = self.increment_transaction_id();
    //
    //         match Tag::get_one_by_custom(self.pool(), "tag", &request.id_tag.to_lowercase()).await {
    //           Ok(tag) => {
    //             info!(tag = request.id_tag, "tag accepted");
    //             debug!(tag = ?tag);
    //
    //             return StartTransactionResponse {
    //               transaction_id,
    //               id_tag_info: IdTagInfo {
    //                 expiry_date: None,
    //                 parent_id_tag: None,
    //                 status: AuthorizationStatus::Accepted,
    //               },
    //             };
    //           }
    //           Err(_) => {
    //             error!(tag = request.id_tag, "tag invalid");
    //             return StartTransactionResponse {
    //               transaction_id,
    //               id_tag_info: IdTagInfo {
    //                 expiry_date: None,
    //                 parent_id_tag: None,
    //                 status: AuthorizationStatus::Invalid,
    //               },
    //             };
    //           }
    //         }
    //       },
    //     )
    //     .await
    //   }
    //   StopTransaction => {
    //     Self::handle_ocpp_request::<StopTransactionRequest, StopTransactionResponse, _, _>(
    //       msg_id,
    //       payload.clone(),
    //       |_request| async move { StopTransactionResponse { id_tag_info: None } },
    //     )
    //     .await
    //   }
    //   StatusNotification => {
    //     Self::handle_ocpp_request::<StatusNotificationRequest, StatusNotificationResponse, _, _>(
    //       msg_id,
    //       payload.clone(),
    //       |_request| async move { StatusNotificationResponse {} },
    //     )
    //     .await
    //   }
    //   MeterValues => {
    //     Self::handle_ocpp_request::<MeterValuesRequest, MeterValuesResponse, _, _>(
    //       msg_id,
    //       payload.clone(),
    //       |_request| async move { MeterValuesResponse {} },
    //     )
    //     .await
    //   }
    //   DiagnosticsStatusNotification => {
    //     Self::handle_ocpp_request::<
    //       DiagnosticsStatusNotificationRequest,
    //       DiagnosticsStatusNotificationResponse,
    //       _,
    //       _,
    //     >(msg_id, payload.clone(), |_request| async move {
    //       DiagnosticsStatusNotificationResponse {}
    //     })
    //     .await
    //   }
    //   FirmwareStatusNotification => {
    //     Self::handle_ocpp_request::<
    //       FirmwareStatusNotificationRequest,
    //       FirmwareStatusNotificationResponse,
    //       _,
    //       _,
    //     >(msg_id, payload.clone(), |_request| async move {
    //       FirmwareStatusNotificationResponse {}
    //     })
    //     .await
    //   }
    //   DataTransfer => {
    //     Self::handle_ocpp_request::<DataTransferRequest, DataTransferResponse, _, _>(
    //       msg_id,
    //       payload.clone(),
    //       |_request| async move {
    //         DataTransferResponse {
    //           status: DataTransferStatus::Accepted,
    //           data: None,
    //         }
    //       },
    //     )
    //     .await
    //   }
    //   _ => anyhow::bail!("Unknown OCPP action to parse payload"), // CSMS → CP
    //                                                               // RemoteStartTransaction,
    //                                                               // RemoteStopTransaction,
    //                                                               // Reset,
    //                                                               // ChangeAvailability,
    //                                                               // ChangeConfiguration,
    //                                                               // GetConfiguration,
    //                                                               // ClearCache,
    //                                                               // UpdateFirmware,
    //                                                               // GetDiagnostics,
    //                                                               // UnlockConnector,
    //                                                               // CancelReservation,
    //                                                               // ReserveNow,
    //                                                               // SetChargingProfile,
    //                                                               // ClearChargingProfile,
    //                                                               // GetCompositeSchedule,
    //                                                               // GetLocalListVersion,
    //                                                               // SendLocalList,
    // }
  }
}
