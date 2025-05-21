use std::{sync::Arc, time::Duration};

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::sync::Mutex;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::http::Request;
use tracing::{error, info};
use tungstenite::{Message, handshake::client::generate_key, http::header::SEC_WEBSOCKET_PROTOCOL};
use url::Url;
use uuid::Uuid;

use crate::{message_generator::MessageGenerator, ocpp::v1_6::{message_generator::Generator, types::OcppAction}};

pub struct WsClientConfig {
  csms_url: Url,
  serial_number: String,
  vendor: String,
  model: String,
}

impl Default for WsClientConfig {
  fn default() -> Self {
    Self {
      csms_url: Url::parse("ws://localhost:3000").unwrap(),
      serial_number: String::from("ocpp-charge-point-simulator"),
      vendor: String::from("ocpp-rust"),
      model: String::from("ocpp-rust-v1"),
    }
  }
}

pub struct WsClientConfigBuilder {
  csms_url: Option<Url>,
  serial_number: Option<String>,
  vendor: Option<String>,
  model: Option<String>,
}

impl WsClientConfigBuilder {
  pub fn new() -> Self {
    Self {
      csms_url: None,
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

  pub fn serial_number(mut self, id: impl Into<String>) -> Self {
    self.serial_number = Some(id.into());
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

  pub fn build(self) -> WsClientConfig {
    let config_default = WsClientConfig::default();

    WsClientConfig {
      csms_url: self.csms_url.unwrap_or(config_default.csms_url),
      serial_number: self.serial_number.unwrap_or(config_default.serial_number),
      vendor: self.vendor.unwrap_or(config_default.vendor),
      model: self.model.unwrap_or(config_default.model),
    }
  }
}

pub struct WsClient {
  config: WsClientConfig,
}

impl WsClient {
  pub fn new(config: WsClientConfig) -> Self {
    Self { config }
  }

  pub async fn run(&mut self) -> Result<()> {
    info!(target: "simulator", "Connecting to CSMS at {}", self.config.csms_url);

    let request = Request::builder()
      .method("GET")
      .uri(self.config.csms_url.to_string())
      .header(
        "Host",
        format!(
          "{}{}",
          self.config.csms_url.host_str().unwrap(),
          self.config.csms_url.port().unwrap()
        ),
      )
      .header(SEC_WEBSOCKET_PROTOCOL, "ocpp1.6")
      .header("Connection", "Upgrade")
      .header("Upgrade", "websocket")
      .header("Sec-WebSocket-Version", "13")
      .header("Sec-WebSocket-Key", generate_key())
      .header("Sec-WebSocket-Protocol", "ocpp1.6")
      .body(())?;

    let (ws_stream, _) = connect_async(request).await?;
    let (mut ws_tx, mut ws_rx) = ws_stream.split();

    let boot_notification = Generator::to_frame(OcppAction::BootNotification, Generator::boot_notification());
    let boot_notification_string = serde_json::to_string(&boot_notification)?;

    // let boot = json!([
    //   2,
    //   Uuid::new_v4().to_string(),
    //   "BootNotification",
    //   {
    //     "chargePointVendor": self.config.vendor,
    //     "chargePointModel": self.config.model
    //   }
    // ]);

    ws_tx
      .send(Message::Text(boot_notification_string.into()))
      .await
      .unwrap();

    let ws_tx_mutex = Arc::new(Mutex::new(ws_tx));
    let ws_tx_mutex_clone = ws_tx_mutex.clone();

    let outbound_task = tokio::spawn(async move {
      loop {
        tokio::time::sleep(Duration::from_secs(10)).await;

        let start_txn = json!([
          2,
          Uuid::new_v4().to_string(),
          "StartTransaction",
          {
            "connectorId": 1,
            "idTag": "ABC123",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "meterStart": 0
          }
        ]);

        let mut ws_tx_guard = ws_tx_mutex_clone.lock().await;
        if let Err(e) = ws_tx_guard
          .send(Message::Text(start_txn.to_string().into()))
          .await
        {
          error!("Failed to send StartTransaction: {e}");
          break;
        }

        info!("StartTransaction sent");
      }
    });

    while let Some(msg) = ws_rx.next().await {
      match msg {
        Ok(Message::Text(text)) => {
          info!("Received: {}", text);
          if text.contains("GetConfiguration") {
            let call_result = json!([
              3,
              "123456",
              {
                "configurationKey": []
              }
            ]);

            info!("Responded to GetConfiguration");
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

    outbound_task.abort();
    info!("Client shutdown");

    Ok(())
  }
}
