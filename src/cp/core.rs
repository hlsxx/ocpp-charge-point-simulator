use crate::{
  config::{ChargePointConfig, GeneralConfig},
  ocpp::{
    message_generator::{MessageBuilderTrait, MessageGeneratorConfig, MessageGeneratorTrait},
    messsage_handler::OcppMessageHandler,
  },
};

use anyhow::Result;
use colored::Colorize;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::{
  select,
  sync::Mutex,
  time::{self, Duration, Instant, interval, sleep},
};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::http::Request;
use tracing::{debug, error, info};
use tungstenite::{
  Message,
  handshake::client::generate_key,
  http::header::{
    CONNECTION, HOST, SEC_WEBSOCKET_KEY, SEC_WEBSOCKET_PROTOCOL, SEC_WEBSOCKET_VERSION, UPGRADE,
  },
};
use url::Url;

use crate::{
  ocpp::types::OcppVersion,
  ocpp::v1_6::{
    message_generator::MessageGenerator as Ocpp16MessageGenerator,
    message_handler::MessageHandler as Ocpp16MessageHandler,
  },
  ocpp::v2_0_1::{
    message_generator::MessageGenerator as Ocpp201MessageGenerator,
    message_handler::MessageHandler as Ocpp201MessageHandler,
  },
  ocpp::v2_1::{
    message_generator::MessageGenerator as Ocpp21MessageGenerator,
    message_handler::MessageHandler as Ocpp21MessageHandler,
  },
};

pub struct ChargePoint {
  general_config: Arc<GeneralConfig>,
  charge_point_config: ChargePointConfig,
}

impl ChargePoint {
  pub fn new(general_config: Arc<GeneralConfig>, charge_point_config: ChargePointConfig) -> Self {
    Self {
      general_config,
      charge_point_config,
    }
  }

  pub async fn run(&mut self) -> Result<()> {
    let connection_url = format!(
      "{}/{}",
      self.general_config.server_url.to_string(),
      self.charge_point_config.id
    );

    info!(target: "simulator", "connecting to CSMS at {}", connection_url.cyan());

    let request = Request::builder()
      .method("GET")
      .uri(&connection_url)
      .header(HOST, connection_url)
      .header(
        SEC_WEBSOCKET_PROTOCOL,
        &self.general_config.ocpp_version.to_string(),
      )
      .header(CONNECTION, "Upgrade")
      .header(UPGRADE, "Websocket")
      .header(SEC_WEBSOCKET_VERSION, "13")
      .header(SEC_WEBSOCKET_KEY, generate_key())
      .header(SEC_WEBSOCKET_PROTOCOL, OcppVersion::V1_6.to_string())
      .body(())?;

    let (ws_stream, _) = connect_async(request).await?;
    let (mut ws_tx, mut ws_rx) = ws_stream.split();

    let (ocpp_message_generator, mut ocpp_message_handler): (
      Box<dyn MessageGeneratorTrait>,
      Box<dyn OcppMessageHandler>,
    ) = match self.general_config.ocpp_version {
      OcppVersion::V1_6 => (
        Box::new(Ocpp16MessageGenerator::new(
          MessageGeneratorConfig::default(),
        )),
        Box::new(Ocpp16MessageHandler::new()),
      ),
      OcppVersion::V2_0_1 => (
        Box::new(Ocpp201MessageGenerator::new(
          MessageGeneratorConfig::default(),
        )),
        Box::new(Ocpp201MessageHandler::new()),
      ),
      OcppVersion::V2_1 => (
        Box::new(Ocpp21MessageGenerator::new(
          MessageGeneratorConfig::default(),
        )),
        Box::new(Ocpp21MessageHandler::new()),
      ),
    };

    let mut heartbeat_interval = interval(Duration::from_secs(
      self.charge_point_config.heartbeat_interval,
    ));

    let mut status_interval = interval(Duration::from_secs(
      self.charge_point_config.status_interval,
    ));

    let mut meter_values_interval = interval(Duration::from_secs(2));

    let mut next_start_tx =
      Instant::now() + Duration::from_secs(self.charge_point_config.start_tx_after);
    let mut stop_tx_deadline: Option<Instant> = None;
    let mut transaction_active = false;

    let _ = sleep(Duration::from_millis(
      self.charge_point_config.boot_delay_interval,
    ))
    .await;

    ws_tx
      .send(Message::Text(
        ocpp_message_generator
          .boot_notification()
          .to_string()
          .into(),
      ))
      .await
      .unwrap();

    loop {
      select! {
        _ = time::sleep_until(next_start_tx), if !transaction_active => {
          let _ = ws_tx.send(
            Message::Text(ocpp_message_generator.start_transaction().to_string().into())
          ).await;

          stop_tx_deadline = Some(Instant::now() + Duration::from_secs(self.charge_point_config.stop_tx_after));
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
            Message::Text(ocpp_message_generator.stop_transaction().to_string().into())
          ).await;

          transaction_active = false;
          stop_tx_deadline = None;
          next_start_tx = Instant::now() + Duration::from_secs(self.charge_point_config.start_tx_after);
        },

        _ = meter_values_interval.tick(), if transaction_active => {
          let _ = ws_tx.send(Message::Text(ocpp_message_generator.meter_values().to_string().into())).await;
        },

        _ = heartbeat_interval.tick() => {
          let _ = ws_tx.send(Message::Text(ocpp_message_generator.heartbeat().to_string().into())).await;
        },

        _ = status_interval.tick() => {
          let _ = ws_tx.send(Message::Text(ocpp_message_generator.status_notification().to_string().into())).await;
        },

        Some(msg) = ws_rx.next() => {
          match msg {
            Ok(Message::Text(text)) => {
              info!("Received: {}", text);

              if let Some(response_message) = ocpp_message_handler.handle_text_message(&text).await? {
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

    // let outbound_task = tokio::spawn(async move {
    //   loop {
    //     tokio::time::sleep(Duration::from_secs(10)).await;
    //
    //     let start_transaction = MessageGenerator::to_frame(
    //       &message_generator,
    //       OcppAction::StartTransaction,
    //       message_generator.start_transaction(),
    //     );
    //
    //     let mut ws_tx_guard = ws_tx_mutex_clone.lock().await;
    //
    //     match ws_tx_guard
    //       .send(Message::Text(start_transaction.to_string().into()))
    //       .await
    //     {
    //       Ok(_) => {
    //         info!("StartTransaction sent");
    //         debug!(?start_transaction);
    //       }
    //       Err(err) => {
    //         error!("Failed to send StartTransaction: {err}");
    //         break;
    //       }
    //     }
    //   }
    // });

    // outbound_task.abort();
    info!("Client shutdown");

    Ok(())
  }
}
