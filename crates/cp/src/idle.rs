use anyhow::Result;
use common::{ChargePointConfig, GeneralConfig};
use futures::SinkExt;
use ocpp::{
  OcppSession,
  handler::{MessageFrame, MessageFrameType},
  types::{AuthorizationStatus, CommonConnectorStatusType, CommonOcppResponse},
  v1_6::{handler::V16MessageHandler, types::OcppAction},
};

use std::{sync::Arc, time::Duration};
use tokio::{
  select,
  time::{interval, sleep},
};
use tungstenite::Message;

use futures_util::StreamExt;
use tracing::{error, info, warn};

use crate::{
  core::{connect, send},
  session::TxnSession,
};

pub struct ChargePointIdle {
  general_config: Arc<GeneralConfig>,
  config: ChargePointConfig,
}

impl ChargePointIdle {
  pub fn new(general_config: Arc<GeneralConfig>, config: ChargePointConfig) -> Self {
    Self {
      general_config,
      config,
    }
  }

  pub async fn run(&mut self) -> Result<()> {
    let ocpp_version = &self.general_config.ocpp_version;

    let ws_stream = connect(self.general_config.clone(), &self.config).await?;
    let (mut ws_tx, mut ws_rx) = ws_stream.split();

    let OcppSession { generator, handler } =
      OcppSession::new(ocpp_version, self.config.clone()).await;

    let mut txn_session = TxnSession::new(
      self.config.txn_meter_values_interval,
      self.config.txn_meter_values_max_count,
    );

    let mut heartbeat_interval = interval(Duration::from_secs(self.config.heartbeat_interval));

    send(&mut ws_tx, generator.boot_notification().await).await?;

    loop {
      select! {
        // Charge point heartbeats
        _ = heartbeat_interval.tick() => send(&mut ws_tx, generator.heartbeat().await).await?,
        // Transaction (meter_values) session
        _ = txn_session.tick(), if txn_session.is_running() => {
          send(&mut ws_tx, generator.meter_values().await).await?;
          txn_session.increment();
          if !txn_session.is_running() {
            send(&mut ws_tx, generator.stop_transaction().await).await?;

            // Sets a connector to an `Available` status
            send(&mut ws_tx, generator.status_notification(
              CommonConnectorStatusType::Available
            ).await).await?;
          }
        },

        // Handles a CSMS messages
        msg = ws_rx.next() => {
          match msg {
            Some(Ok(Message::Text(text_msg))) => {
              match handler.parse_raw_ocpp_msg(&text_msg)? {
                MessageFrameType::V1_6(ocpp_msg_frame) => {
                  match ocpp_msg_frame {
                    MessageFrame::Call {
                      action,
                      payload,
                      ..
                    } => {
                      match action {
                        OcppAction::ChangeConfiguration => {
                          let change_configuration_payload = V16MessageHandler::parse_change_configuration_payload(payload)?;
                          match change_configuration_payload.key.as_str() {
                            // 🔌 Core / Timing

                            "HeartbeatInterval" => {
                              generator.heartbeat_interval(change_configuration_payload.value.parse::<u32>()?).await
                            },
                            "MeterValueSampleInterval" => {
                              generator.meter_value_sample_interval(change_configuration_payload.value.parse::<u32>()?).await
                            },
                            "ClockAlignedDataInterval" => {
                              generator.clock_aligned_data_interval(change_configuration_payload.value.parse::<u32>()?).await
                            },
                            "ConnectionTimeOut" => {
                              generator.connection_timeout(change_configuration_payload.value.parse::<u32>()?).await
                            },
                            "ResetRetries" => {
                              generator.reset_retries(change_configuration_payload.value.parse::<u32>()?).await
                            },
                            "WebSocketPingInterval" => {
                              generator.websocket_ping_interval(change_configuration_payload.value.parse::<u32>()?).await
                            },

                            // ⚡ Metering

                            "MeterValuesSampledData" => {
                              generator.meter_values_sampled_data(change_configuration_payload.value.clone()).await
                            },
                            "MeterValuesAlignedData" => {
                              generator.meter_values_aligned_data(change_configuration_payload.value.clone()).await
                            },
                            "StopTxnSampledData" => {
                              generator.stop_txn_sampled_data(change_configuration_payload.value.clone()).await
                            },
                            "StopTxnAlignedData" => {
                              generator.stop_txn_aligned_data(change_configuration_payload.value.clone()).await
                            },

                            // 🔄 Transaction behavior

                            "TransactionMessageAttempts" => {
                              generator.transaction_message_attempts(change_configuration_payload.value.parse::<u32>()?).await
                            },
                            "TransactionMessageRetryInterval" => {
                              generator.transaction_message_retry_interval(change_configuration_payload.value.parse::<u32>()?).await
                            },
                            "MaxEnergyOnInvalidId" => {
                              generator.max_energy_on_invalid_id(change_configuration_payload.value.parse::<u32>()?).await
                            },

                            // 🔐 Authorization

                            "AuthorizeRemoteTxRequests" => {
                              generator.authorize_remote_tx_requests(change_configuration_payload.value.parse::<bool>()?).await
                            },
                            "StopTransactionOnEVSideDisconnect" => {
                              generator.stop_transaction_on_ev_side_disconnect(change_configuration_payload.value.parse::<bool>()?).await
                            },
                            "StopTransactionOnInvalidId" => {
                              generator.stop_transaction_on_invalid_id(change_configuration_payload.value.parse::<bool>()?).await
                            },
                            "AllowOfflineTxForUnknownId" => {
                              generator.allow_offline_tx_for_unknown_id(change_configuration_payload.value.parse::<bool>()?).await
                            },
                            "LocalAuthorizeOffline" => {
                              generator.local_authorize_offline(change_configuration_payload.value.parse::<bool>()?).await
                            },
                            "LocalPreAuthorize" => {
                              generator.local_pre_authorize(change_configuration_payload.value.parse::<bool>()?).await
                            },
                            "AuthorizationCacheEnabled" => {
                              generator.authorization_cache_enabled(change_configuration_payload.value.parse::<bool>()?).await
                            },

                            // 💳 Local Authorization List

                            "LocalAuthListEnabled" => {
                              generator.local_auth_list_enabled(change_configuration_payload.value.parse::<bool>()?).await
                            },
                            "LocalAuthListVersion" => {
                              generator.local_auth_list_version(change_configuration_payload.value.parse::<i32>()?).await
                            },
                            "SendLocalListMaxLength" => {
                              generator.send_local_list_max_length(change_configuration_payload.value.parse::<u32>()?).await
                            },
                            "LocalAuthListMaxLength" => {
                              generator.local_auth_list_max_length(change_configuration_payload.value.parse::<u32>()?).await
                            },

                            // 🔌 Connector / Hardware

                            "NumberOfConnectors" => {
                              generator.number_of_connectors(change_configuration_payload.value.parse::<u32>()?).await
                            },
                            "ConnectorPhaseRotation" => {
                              generator.connector_phase_rotation(change_configuration_payload.value.clone()).await
                            },

                            // ⚡ Smart Charging

                            "ChargeProfileMaxStackLevel" => {
                              generator.charge_profile_max_stack_level(change_configuration_payload.value.parse::<u32>()?).await
                            },
                            "ChargingScheduleAllowedChargingRateUnit" => {
                              generator.charging_schedule_allowed_charging_rate_unit(change_configuration_payload.value.clone()).await
                            },
                            "ChargingScheduleMaxPeriods" => {
                              generator.charging_schedule_max_periods(change_configuration_payload.value.parse::<u32>()?).await
                            },

                            // 📊 Limits / Misc

                            "GetConfigurationMaxKeys" => {
                              generator.get_configuration_max_keys(change_configuration_payload.value.parse::<u32>()?).await
                            },
                            key => error!("ChangeConfiguration unknown key {key}"),
                          }
                        },
                        OcppAction::RemoteStartTransaction => {
                          let action_payload = V16MessageHandler::parse_remote_start_transaction_payload(payload)?;
                          send(&mut ws_tx, generator.authorize(Some(&action_payload.id_tag)).await).await?;
                        },
                        OcppAction::RemoteStopTransaction => {
                          txn_session.stop();
                          send(&mut ws_tx, generator.stop_transaction().await).await?;

                          // Sets a connector to an `Available` status
                          send(&mut ws_tx, generator.status_notification(
                            CommonConnectorStatusType::Available
                          ).await).await?;
                        },
                        _ => warn!("Unknown action {}", action)
                      }
                    },
                    MessageFrame::CallResult {
                      msg_id,
                      payload
                    } => {
                      if let Some(common_ocpp_msg) = handler.handle_call_result(&msg_id, &payload).await? {
                        match common_ocpp_msg {
                          CommonOcppResponse::Authorize { status } => {
                            match status {
                              AuthorizationStatus::Accepted => {
                                send(&mut ws_tx, generator.start_transaction().await).await?;
                              },
                              AuthorizationStatus::Blocked |
                              AuthorizationStatus::Expired |
                              AuthorizationStatus::Invalid => {
                                warn!("Authorization rejected: {:?}", status);
                              },
                              AuthorizationStatus::ConcurrentTx => {
                                warn!("Concurrent transaction in progress");
                              },
                            }
                          },
                          CommonOcppResponse::StartTransaction { .. } => {
                            // Sets a connector to an `Preparing` status
                            send(&mut ws_tx, generator.status_notification(
                              CommonConnectorStatusType::Preparing
                            ).await).await?;

                            // Simulates HW connector delay
                            sleep(Duration::from_secs(5)).await;

                            // Sets a connector to an `Charging` status
                            send(&mut ws_tx, generator.status_notification(
                              CommonConnectorStatusType::Charging
                            ).await).await?;

                            txn_session.start();
                          },
                          _ => {}
                        }
                      }
                    },
                    _ => {}
                  }
                },
                #[allow(unused)]
                MessageFrameType::V2_0_1(ocpp_msg_frame) => {
                  error!("Not implemented yet")
                }
                #[allow(unused)]
                MessageFrameType::V2_1(ocpp_msg_frame) => {
                  error!("Not implemented yet")
                }
              }
            },
            Some(other_msg) => warn!("Another message {other_msg:?}"),
            None => break
          }
        }

        _ = tokio::signal::ctrl_c() => break
      }
    }

    ws_tx.close().await?;
    info!("Client shutdown");

    Ok(())
  }
}
