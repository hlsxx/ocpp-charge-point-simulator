use crate::{
  config::{ChargePointConfig, GeneralConfig},
  ocpp::message_generator::{MessageGeneratorTrait, MessageGeneratorConfig},
};

use anyhow::Result;
use colored::Colorize;
use futures_util::{SinkExt, StreamExt};
use std::{sync::Arc};
use tokio::{
  select, sync::Mutex, time::{self, interval, sleep, Duration, Instant}
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
  ocpp::v1_6::{message_generator::MessageGenerator as Ocpp16MessageGenerator},
  ocpp::v2_0_1::{message_generator::MessageGenerator as Ocpp201MessageGenerator},
  // v2_1::{message::MessageGenerator as V2_1MessageGenerator, types::OcppAction},
};

pub struct ChargerConfig {
  csms_url: Url,
  charge_point_id: String,
  serial_number: String,
  vendor: String,
  model: String,
}

impl Default for ChargerConfig {
  fn default() -> Self {
    Self {
      csms_url: Url::parse("ws://localhost:3000").unwrap(),
      charge_point_id: format!("CP{}", rand::random_range(100_000..999_999)),
      serial_number: String::from("ocpp-charge-point-simulator"),
      vendor: String::from("ocpp-rust"),
      model: String::from("ocpp-rust-v1"),
    }
  }
}

pub struct ChargerConfigBuilder {
  csms_url: Option<Url>,
  charge_point_id: Option<String>,
  serial_number: Option<String>,
  vendor: Option<String>,
  model: Option<String>,
}

impl ChargerConfigBuilder {
  pub fn new() -> Self {
    Self {
      csms_url: None,
      charge_point_id: None,
      serial_number: None,
      vendor: None,
      model: None,
    }
  }

  pub fn csms_url(mut self, url_string: impl Into<String>) -> Self {
    if let Ok(url) = Url::parse(&url_string.into()) {
      self.csms_url = Some(url);
    }
    self
  }

  pub fn charge_point_id(mut self, id: impl Into<String>) -> Self {
    self.charge_point_id = Some(id.into());
    self
  }

  pub fn serial_number(mut self, serial_number: impl Into<String>) -> Self {
    self.serial_number = Some(serial_number.into());
    self
  }

  pub fn vendor(mut self, vendor: impl Into<String>) -> Self {
    self.vendor = Some(vendor.into());
    self
  }

  pub fn model(mut self, model: impl Into<String>) -> Self {
    self.model = Some(model.into());
    self
  }

  pub fn build(self) -> ChargerConfig {
    let config_default = ChargerConfig::default();

    ChargerConfig {
      csms_url: self.csms_url.unwrap_or(config_default.csms_url),
      charge_point_id: self
        .charge_point_id
        .unwrap_or(config_default.charge_point_id),
      serial_number: self.serial_number.unwrap_or(config_default.serial_number),
      vendor: self.vendor.unwrap_or(config_default.vendor),
      model: self.model.unwrap_or(config_default.model),
    }
  }
}

pub struct Charger {
  general_config: Arc<GeneralConfig>,
  charge_point_config: ChargePointConfig,
}

impl Charger {
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

    let message_generator: Box<dyn MessageGeneratorTrait> = match self.general_config.ocpp_version {
      OcppVersion::V1_6 => Box::new(Ocpp16MessageGenerator::new(MessageGeneratorConfig::default())),
      OcppVersion::V2_0_1 => Box::new(Ocpp201MessageGenerator::new(MessageGeneratorConfig::default())),
      OcppVersion::V2_1 => Box::new(Ocpp201MessageGenerator::new(MessageGeneratorConfig::default())),
    };

    let mut heartbeat_interval = interval(Duration::from_secs(
      self.charge_point_config.heartbeat_interval,
    ));

    let mut status_interval = interval(Duration::from_secs(
      self.charge_point_config.status_interval,
    ));

    let mut meter_values_interval = interval(Duration::from_secs(2));

    let mut next_start_tx = Instant::now() + Duration::from_secs(self.charge_point_config.start_tx_after);
    let mut stop_tx_deadline: Option<Instant> = None;
    let mut transaction_active = false;

    let _ = sleep(Duration::from_millis(self.charge_point_config.boot_delay_interval)).await;

    ws_tx
      .send(Message::Text(message_generator.boot_notification().to_string().into()))
      .await
      .unwrap();

    loop {
      select! {
        _ = time::sleep_until(next_start_tx), if !transaction_active => {
          let _ = ws_tx.send(
            Message::Text(message_generator.start_transaction().to_string().into())
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
            Message::Text(message_generator.stop_transaction().to_string().into())
          ).await;

          transaction_active = false;
          stop_tx_deadline = None;
          next_start_tx = Instant::now() + Duration::from_secs(self.charge_point_config.start_tx_after);
        },

        _ = meter_values_interval.tick(), if transaction_active => {
          let _ = ws_tx.send(Message::Text(message_generator.meter_values().to_string().into())).await;
        },

        _ = heartbeat_interval.tick() => {
          let _ = ws_tx.send(Message::Text(message_generator.heartbeat().to_string().into())).await;
        },

        _ = status_interval.tick() => {
          let _ = ws_tx.send(Message::Text(message_generator.status_notification().to_string().into())).await;
        },

        Some(msg) = ws_rx.next() => {
          match msg {
            Ok(Message::Text(text)) => {
              info!("Received: {}", text);
              // if text.contains("GetConfiguration") {
              //   let call_result = json!([
              //     3,
              //     "123456",
              //     {
              //       "configurationKey": []
              //     }
              //   ]);
              //
              //   info!("Responded to GetConfiguration");
              // }
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
