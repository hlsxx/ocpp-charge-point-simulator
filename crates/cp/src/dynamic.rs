use anyhow::Result;
use common::{ChargePointConfig, GeneralConfig};
use std::sync::Arc;

use tokio::{
  select,
  time::{self, Duration, Instant, interval, sleep},
};

use ocpp::{
  create_ocpp_handlers, msg_generator::MessageGenerator, msg_handler::MessageHandler,
  types::CommonConnectorStatusType,
};

use futures_util::{SinkExt, StreamExt};
use tracing::{error, info};
use tungstenite::Message;

#[cfg(feature = "ocpp1_6")]
use ocpp::v1_6::{msg_generator::V16MessageGenerator, msg_handler::V16MessageHandler};

#[cfg(feature = "ocpp2_0_1")]
use ocpp::v2_0_1::{msg_generator::V201MessageGenerator, msg_handler::V201MessageHandler};

#[cfg(feature = "ocpp2_1")]
use ocpp::v2_1::{msg_generator::V21MessageGenerator, msg_handler::V21MessageHandler};

use crate::ChargePointClient;

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
    let ws_stream = ChargePointClient::connect(&self.general_config, &self.config).await?;
    let (mut ws_tx, mut ws_rx) = ws_stream.split();

    let (msg_generator, mut msg_handler) =
      create_ocpp_handlers(self.general_config.ocpp_version.clone());

    let mut heartbeat_interval = interval(Duration::from_secs(self.config.heartbeat_interval));

    // let mut status_interval = interval(Duration::from_secs(
    //   self.charge_point_config.status_interval,
    // ));

    let mut meter_values_interval = interval(Duration::from_secs(2));

    let mut next_start_tx = Instant::now() + Duration::from_secs(self.config.start_tx_after);
    let mut stop_tx_deadline: Option<Instant> = None;
    let mut transaction_active = false;

    let _ = sleep(Duration::from_millis(self.config.boot_delay_interval)).await;

    ws_tx
      .send(Message::Text(
        msg_generator.boot_notification().await.to_string().into(),
      ))
      .await
      .unwrap();

    loop {
      select! {
        _ = time::sleep_until(next_start_tx), if !transaction_active => {
          let _ = ws_tx.send(
            Message::Text(msg_generator.start_transaction().await.to_string().into())
          ).await;

          let _ = ws_tx.send(
            Message::Text(
              msg_generator.status_notification(CommonConnectorStatusType::Preparing).await.to_string().into()
            )
          ).await;

          // TODO: This is timeout for assign transaction_id from the CSMS call result
          tokio::time::sleep(Duration::from_secs(5)).await;

          let _ = ws_tx.send(
            Message::Text(
              msg_generator.status_notification(CommonConnectorStatusType::Charging).await.to_string().into()
            )
          ).await;

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
          let _ = ws_tx.send(
            Message::Text(msg_generator.stop_transaction().await.to_string().into())
          ).await;

          let _ = ws_tx.send(
            Message::Text(
              msg_generator.status_notification(CommonConnectorStatusType::Available).await.to_string().into()
            )
          ).await;

          transaction_active = false;
          stop_tx_deadline = None;
          next_start_tx = Instant::now() + Duration::from_secs(self.config.start_tx_after);
        },

        _ = meter_values_interval.tick(), if transaction_active => {
          let meter_values_string = msg_generator.meter_values().await.to_string();

          if meter_values_string != "null" {
            let _ = ws_tx.send(Message::Text(meter_values_string.into())).await;
          }
        },

        _ = heartbeat_interval.tick() => {
          let _ = ws_tx.send(Message::Text(msg_generator.heartbeat().await.to_string().into())).await;
        },

        // _ = status_interval.tick() => {
        //   let _ = ws_tx.send(Message::Text(msg_generator.status_notification().await.to_string().into())).await;
        // },

        Some(msg) = ws_rx.next() => {
          match msg {
            Ok(Message::Text(text)) => {
              info!("Received: {}", text);

              // TODO: Parse message
              //let shared_data = shared_data.write().await;

              if let Some(response_message) = msg_handler.handle_text_message(&text).await? {
                ws_tx
                  .send(Message::Text(response_message.clone().into()))
                  .await?;
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
      }
    }

    info!("Client shutdown");

    Ok(())
  }
}
