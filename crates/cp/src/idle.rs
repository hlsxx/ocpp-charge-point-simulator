use anyhow::Result;
use common::{ChargePointConfig, GeneralConfig};
use futures::SinkExt;
use ocpp::{
  OcppSession,
  msg_handler::{MessageFrame, MessageFrameType},
  types::{AuthorizationStatus, CommonConnectorStatusType, CommonOcppResponse},
  v1_6::{msg_handler::V16MessageHandler, types::OcppAction},
};

use std::{sync::Arc, time::Duration};
use tokio::{
  select,
  time::{interval, sleep},
};
use tungstenite::Message;

use futures_util::StreamExt;
use tracing::{debug, error, info, warn};

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

    let OcppSession { generator, handler } = OcppSession::new(ocpp_version, self.config.clone());

    let mut txn_session = TxnSession::new(
      self.config.txn_meter_values_interval,
      self.config.txn_meter_values_max_count,
    );

    let mut heartbeat_interval = interval(Duration::from_secs(self.config.heartbeat_interval));

    send(&mut ws_tx, generator.boot_notification().await).await?;

    // Track state between messages
    let mut pending_id_tag: Option<String> = None;

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
              if let MessageFrameType::V1_6(ocpp_msg_frame) = handler.parse_ocpp_message(&text_msg)? {
                match ocpp_msg_frame {
                  MessageFrame::Call {
                    action,
                    payload,
                    ..
                  } => {
                    match action {
                      OcppAction::RemoteStartTransaction => {
                        let action_payload = V16MessageHandler::parse_remote_start_transaction_payload(payload)?;
                        pending_id_tag = Some(action_payload.id_tag.clone());
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
                      _ => warn!("Other action")
                    }
                  },
                  MessageFrame::CallResult {
                    msg_id,
                    payload
                  } => {
                    debug!("Got call result {} {:?}", msg_id, payload);

                    if let Some(common_ocpp_msg) = handler.handle_call_result(&msg_id, &payload).await? {
                      match common_ocpp_msg {
                        CommonOcppResponse::Authorize { status } => {
                          match status {
                            AuthorizationStatus::Accepted => {
                              if let Some(id_tag) = pending_id_tag.take() {
                                send(&mut ws_tx, generator.start_transaction(Some(&id_tag)).await).await?;
                              }
                            },
                            AuthorizationStatus::Blocked |
                            AuthorizationStatus::Expired |
                            AuthorizationStatus::Invalid => {
                              warn!("Authorization rejected: {:?}", status);
                              pending_id_tag = None;
                            },
                            AuthorizationStatus::ConcurrentTx => {
                              warn!("Concurrent transaction in progress");
                              pending_id_tag = None;
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
              } else {
                error!("Unknown message");
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
