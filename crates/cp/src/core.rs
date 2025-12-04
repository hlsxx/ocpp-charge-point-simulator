use std::str::FromStr;

use anyhow::Result;
use colored::Colorize;
use common::{ChargePointConfig, GeneralConfig, OcppVersion};
use http::Uri;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use tracing::info;
use tungstenite::ClientRequestBuilder;

pub struct ChargePointClient;

impl ChargePointClient {
  pub async fn connect(
    general_config: &GeneralConfig,
    config: &ChargePointConfig,
  ) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    let uri = Uri::from_str(&format!("{}/{}", general_config.server_url, config.id))?;

    info!(target: "simulator", "connecting to CSMS at {}", uri.to_string().cyan());

    let request = ClientRequestBuilder::new(uri)
      .with_header("Authorization", "Basic dHNzMDR0ZXN0OnBhc3N3b3Jk")
      .with_sub_protocol(OcppVersion::V1_6.to_string());

    let (ws_stream, _) = connect_async(request).await?;

    Ok(ws_stream)
  }
}
