use std::{sync::Arc, time::Duration};

use serde_json::json;
use tokio::sync::Mutex;
use tokio_tungstenite::connect_async;
use tracing::{error, info};
use anyhow::Result;
use url::Url;
use futures_util::{SinkExt, StreamExt};
use uuid::Uuid;
use tungstenite::{handshake::client::generate_key, http::header::SEC_WEBSOCKET_PROTOCOL, Message};
use tokio_tungstenite::tungstenite::http::Request;

pub struct WsClientConfig {
  csms_url: String,
  charge_point_id: String,
  vendor: String,
  model: String,
}

impl Default for WsClientConfig {
  fn default() -> Self {
    Self {
      csms_url: String::from("ws://localhost:3000"),
      charge_point_id: String::from("ocpp-charge-point-simulator"),
      vendor: String::from("ocpp-rust"),
      model: String::from("ocpp-rust-v1"),
    }
  }
}

pub struct WsClientConfigBuilder {
  csms_url: Option<String>,
  charge_point_id: Option<String>,
  vendor: Option<String>,
  model: Option<String>,
}

impl WsClientConfigBuilder {
  pub fn new() -> Self {
    Self {
      csms_url: None,
      charge_point_id: None,
      vendor: None,
      model: None
    }
  }

  pub fn csms_url(mut self, url: impl Into<String>) -> Self { self.csms_url = Some(url.into()); self }

  pub fn charge_point_id(mut self, id: impl Into<String>) -> Self {
    self.charge_point_id = Some(id.into());
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
      charge_point_id: self.charge_point_id.unwrap_or(config_default.charge_point_id),
      vendor: self.vendor.unwrap_or(config_default.vendor),
      model: self.model.unwrap_or(config_default.model),
    }
  }
}

pub struct WsClient {
  config: WsClientConfig
}

impl WsClient {
  pub fn new(config: WsClientConfig) -> Self {
    Self {
      config
    }
  }

  pub async fn run(&mut self) -> Result<()> {
    info!(target: "simulator", "Connecting to CSMS at {}", self.config.csms_url);

    let url = Url::parse(&self.config.csms_url)?;

    let request = Request::builder()
      .method("GET")
      .uri(url.to_string())
      .header("Host", format!("{}{}", url.host_str().unwrap(), url.port().unwrap()))
      .header(SEC_WEBSOCKET_PROTOCOL, "ocpp1.6")
      .header("Connection", "Upgrade")
      .header("Upgrade", "websocket")
      .header("Sec-WebSocket-Version", "13")
      .header("Sec-WebSocket-Key", generate_key())
      .header("Sec-WebSocket-Protocol", "ocpp1.6")
      .body(())?;

    let (ws_stream, _) = connect_async(request).await?;
    let (mut ws_tx, mut ws_rx) = ws_stream.split();

    let boot = json!([
      2,
      Uuid::new_v4().to_string(),
      "BootNotification",
      {
        "chargePointVendor": self.config.vendor,
        "chargePointModel": self.config.model
      }
    ]);

    ws_tx.send(Message::Text(boot.to_string().into()))
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
        if let Err(e) = ws_tx_guard.send(Message::Text(start_txn.to_string().into())).await {
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

