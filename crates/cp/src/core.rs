use std::{fmt::Display, str::FromStr, sync::Arc};

use anyhow::Result;
use colored::Colorize;
use common::{ChargePointConfig, GeneralConfig, OcppVersion};
use futures::SinkExt;
use http::Uri;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use tracing::info;
use tungstenite::{ClientRequestBuilder, Message};

pub async fn connect<'a>(
  general_config: Arc<GeneralConfig>,
  cp_config: &'a ChargePointConfig,
) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
  let uri = Uri::from_str(&format!(
    "{}/{}",
    general_config.server_url.trim_end_matches('/'),
    cp_config.id
  ))?;

  info!(target: "simulator", "connecting to CSMS at {}", uri.to_string().cyan());

  let request = ClientRequestBuilder::new(uri)
    .with_header("Authorization", &cp_config.auth_header)
    .with_sub_protocol(OcppVersion::V1_6.to_string());

  let (ws_stream, _) = connect_async(request).await?;

  Ok(ws_stream)
}

pub async fn send<S>(ws_tx: &mut S, msg: impl Display) -> Result<()>
where
  S: SinkExt<Message, Error = tungstenite::Error> + Unpin,
{
  ws_tx.send(text(msg)).await?;
  Ok(())
}

pub fn text(msg: impl Display) -> Message {
  Message::Text(msg.to_string().into())
}
