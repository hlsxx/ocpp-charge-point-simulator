use anyhow::Result;
use common::{ChargePointConfig, GeneralConfig};
use std::sync::Arc;

use tokio::{
  select,
  time::{self, Duration, Instant, interval, sleep},
};

use ocpp::{create_ocpp_handlers, types::CommonConnectorStatusType};

use futures_util::{SinkExt, StreamExt};
use tracing::{error, info};
use tungstenite::Message;

use crate::core::{connect, send};

pub struct ChargePointDynamic {
  general_config: Arc<GeneralConfig>,
  config: ChargePointConfig,
}

impl ChargePointDynamic {
  pub fn new(general_config: Arc<GeneralConfig>, config: ChargePointConfig) -> Self {
    Self {
      general_config,
      config,
    }
  }

  pub async fn run(&mut self) -> Result<()> {
    let ws_stream = connect(&self.general_config, &self.config).await?;
    let (mut ws_tx, mut ws_rx) = ws_stream.split();

    let (msg_generator, mut msg_handler) =
      create_ocpp_handlers(&self.general_config.ocpp_version, &self.config);

    let mut heartbeat_interval = interval(Duration::from_secs(self.config.heartbeat_interval));
    let mut meter_values_interval =
      interval(Duration::from_secs(self.config.txn_meter_values_interval));

    let mut next_start_tx = Instant::now() + Duration::from_secs(self.config.start_tx_after);
    let mut stop_tx_deadline: Option<Instant> = None;
    let mut transaction_active = false;

    let _ = sleep(Duration::from_millis(self.config.boot_delay_interval)).await;

    send(&mut ws_tx, msg_generator.boot_notification().await).await?;

    loop {
      select! {
        _ = time::sleep_until(next_start_tx), if !transaction_active => {
          // Sets a connector to a `Preparing` status
          send(&mut ws_tx, msg_generator.status_notification(
            CommonConnectorStatusType::Preparing
          ).await).await?;

          send(&mut ws_tx, msg_generator.start_transaction(None).await).await?;

          // Simulate a HW timeout
          tokio::time::sleep(Duration::from_secs(5)).await;

          // Sets a connector to a `Charging` status
          send(&mut ws_tx, msg_generator.status_notification(
            CommonConnectorStatusType::Charging
          ).await).await?;

          stop_tx_deadline = Some(Instant::now() + Duration::from_secs(self.config.stop_tx_after));
          transaction_active = true;
        },

        _ = async {
          if let Some(deadline) = stop_tx_deadline {
            time::sleep_until(deadline).await;
          } else {
            futures::future::pending::<()>().await;
          }
        }, if stop_tx_deadline.is_some() => {
          send(&mut ws_tx, msg_generator.stop_transaction().await).await?;

          // Sets a connector to an `Available` status
          send(&mut ws_tx, msg_generator.status_notification(
            CommonConnectorStatusType::Available
          ).await).await?;

          transaction_active = false;
          stop_tx_deadline = None;
          next_start_tx = Instant::now() + Duration::from_secs(self.config.start_tx_after);
        },

        _ = meter_values_interval.tick(), if transaction_active => {
          let meter_value = msg_generator.meter_values().await;
          if !meter_value.is_null() {
            send(&mut ws_tx, meter_value.to_string()).await?;
          }
        },

        _ = heartbeat_interval.tick() => {
          send(&mut ws_tx, msg_generator.heartbeat().await).await?;
        },

        Some(msg) = ws_rx.next() => {
          match msg {
            Ok(Message::Text(text)) => {
              if let Some(response_message) = msg_handler.handle_text_message(&text).await? {
                send(&mut ws_tx, response_message.clone()).await?;
              }
            }
            Ok(Message::Close(_)) => {
              info!("CSMS closed connection");
              break;
            }
            Err(e) => {
              error!("WebSocket error: {e}");
              break;
            }
            _ => {}
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
