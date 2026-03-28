use anyhow::Result;
use common::{ChargePointConfig, GeneralConfig};
use futures::SinkExt;
use ocpp::{
  create_ocpp_handlers,
  msg_handler::{MessageFrame, MessageFrameType},
  types::CommonOcppAction,
  v1_6::{msg_handler::V16MessageHandler, types::OcppAction},
};

use std::{sync::Arc, time::Duration};
use tokio::{
  select,
  time::{interval, sleep},
};
use tungstenite::Message;

use futures_util::StreamExt;
use tracing::{error, info, warn};

use crate::{session::TxnSession, utils::MessageBuilder};

use super::core::ChargePointClient;

/// An `idle mode` charge point
///
/// Represents an `idle` mode charge point
pub struct ChargePointIdle {
  /// General config
  general_config: Arc<GeneralConfig>,

  // Specific chage point config
  config: ChargePointConfig,
}

impl ChargePointIdle {
  pub fn new(general_config: Arc<GeneralConfig>, config: ChargePointConfig) -> Self {
    Self {
      general_config,
      config,
    }
  }

  /// Runs a charge point in `idle mode` that sends messages at specific intervals to the CSMS server.
  /// In idle mode, the charge point sends and also listens for new messages.
  pub async fn run(&mut self) -> Result<()> {
    let ws_stream = ChargePointClient::connect(&self.general_config, &self.config).await?;
    let (mut ws_tx, mut ws_rx) = ws_stream.split();

    let (msg_generator, msg_handler) =
      create_ocpp_handlers(self.general_config.ocpp_version.clone());

    let mut txn_session = TxnSession::new(
      self.config.txn_meter_values_interval,
      self.config.txn_meter_values_max_count,
    );

    let mut heartbeat_interval = interval(Duration::from_secs(self.config.heartbeat_interval));

    loop {
      select! {
        // Charge point heart beats
        _ = heartbeat_interval.tick() => {
          ws_tx.send(MessageBuilder::text(msg_generator.heartbeat().await)).await?;
        },

        // Transaction (meter_values) session
        _ = txn_session.tick(), if txn_session.is_running() => {
          let meter_values = msg_generator.meter_values().await;
          ws_tx.send(MessageBuilder::text(meter_values)).await?;

          txn_session.increment();

          if !txn_session.is_running() {
            // Transaction stop
            let stop_transaction = msg_generator.stop_transaction().await;
            ws_tx.send(MessageBuilder::text(stop_transaction)).await?;

            // Sets a connector as available
            let connector_charging = msg_generator.status_notification(ocpp::types::CommonConnectorStatusType::Available).await;
            ws_tx.send(MessageBuilder::text(connector_charging)).await?;
          }
        },

        // Handles a CSMS messages
        msg = ws_rx.next() => {
          match msg {
            Some(Ok(Message::Text(text_msg))) => {
              if let MessageFrameType::V1_6(ocpp_msg_frame) = msg_handler.parse_ocpp_message(&text_msg)? {
                match ocpp_msg_frame {
                  MessageFrame::Call {
                    action,
                    payload,
                    ..
                  } => {
                    match action {
                      OcppAction::RemoteStartTransaction => {
                        let action_payload = V16MessageHandler::parse_remote_start_transaction_payload(payload)?;

                        // Transaction begin
                        let start_transaction = msg_generator.start_transaction(Some(&action_payload.id_tag)).await;
                        ws_tx.send(MessageBuilder::text(start_transaction)).await?;

                        // Sets a connector as preparing
                        let connector_preparing = msg_generator.status_notification(ocpp::types::CommonConnectorStatusType::Preparing).await;
                        ws_tx.send(MessageBuilder::text(connector_preparing)).await?;
                      },
                      OcppAction::RemoteStopTransaction => {
                        txn_session.stop();

                        let stop_transaction = msg_generator.stop_transaction().await;
                        ws_tx.send(MessageBuilder::text(stop_transaction)).await?;

                        let connector_available = msg_generator.status_notification(ocpp::types::CommonConnectorStatusType::Available).await;
                        ws_tx.send(MessageBuilder::text(connector_available)).await?;
                      },
                      _ => warn!("Other action")
                    }
                  },
                  MessageFrame::CallResult {
                    msg_id,
                    payload
                  } => {
                    println!("Got call result {} {:?}", msg_id, payload);
                    match msg_handler.handle_call_result(&msg_id, &payload).await? {
                      Some(common_ocpp_msg) => {
                          match common_ocpp_msg {
                            CommonOcppAction::StartTransaction => {
                              // Simulates HW connector delay
                              sleep(Duration::from_secs(3)).await;

                              // Sets a connector as charging
                              let connector_charging = msg_generator.status_notification(ocpp::types::CommonConnectorStatusType::Charging).await;
                              ws_tx.send(MessageBuilder::text(connector_charging)).await?;

                              txn_session.start();
                            },
                            _ => {}
                          }
                      },
                      None => {}
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
