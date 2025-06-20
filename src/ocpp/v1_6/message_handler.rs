use std::{
  fmt::Debug,
  str::FromStr
};

use super::types::{OcppMessageFrame, OcppAction};
use crate::ocpp::{messsage_handler::OcppMessageHandler, types::OcppMessageFrameType};
use anyhow::Result;
use async_trait::async_trait;
use rust_ocpp::v1_6::{
  messages::{
    get_configuration::{GetConfigurationRequest, GetConfigurationResponse},
  }
};

use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use tracing::{debug, error, info, span, warn, Level, Span};
use super::types::{DisplayMeasurand, DisplayUnitOfMeasure, DisplayReadingContext, DisplayPhase, DisplayLocation, DisplayValueFormat};

pub struct MessageHandler {}

impl MessageHandler {
  pub fn new() -> Self {
    Self {}
  }
}

#[async_trait]
impl OcppMessageHandler for MessageHandler {
  fn parse_ocpp_message(&self, text: &str) -> Result<OcppMessageFrameType> {
    let arr: Vec<Value> = serde_json::from_str(&text)?;

    match arr.get(0).and_then(|v| v.as_u64()) {
      Some(2) => {
        let msg_id = arr[1].as_str().unwrap_or("").to_string();
        let action_string = arr[2].as_str().unwrap_or("").to_string();
        let payload = arr[3].clone();

        let action = OcppAction::from_str(action_string.as_str())
          .map_err(|err| anyhow::anyhow!("Invalid OCPP action: {}", err))?;

        Ok(OcppMessageFrameType::V1_6(OcppMessageFrame::Call {
          msg_id,
          action,
          payload,
        }))
      }
      Some(3) => {
        let msg_id = arr[1].as_str().unwrap_or("").to_string();
        let payload = arr[2].clone();

        Ok(OcppMessageFrameType::V1_6(OcppMessageFrame::CallResult {
          msg_id,
          payload,
        }))
      }
      Some(4) => {
        let msg_id = arr[1].as_str().unwrap_or("").to_string();
        let error_code = arr[2].as_str().unwrap_or("").to_string();
        let description = arr[3].as_str().unwrap_or("").to_string();

        Ok(OcppMessageFrameType::V1_6(OcppMessageFrame::CallError {
          msg_id,
          error_code,
          description,
        }))
      }
      _ => anyhow::bail!("Unknown OCPP v1.6 message type"),
    }
  }

  async fn handle_text_message(&mut self, text: &str) -> Result<Option<String>> {
    if let OcppMessageFrameType::V1_6(ocpp_message) = self.parse_ocpp_message(text)? {
      match ocpp_message {
        OcppMessageFrame::Call {
          msg_id,
          action,
          payload,
        } => {
          info!("🔌 [🔵 Call] {}", action);
          debug!(?action, msg_id, ?payload);
          return self.handle_call(&msg_id, &action, &payload).await;
        }
        OcppMessageFrame::CallResult { msg_id, payload } => {
          info!("🔌 [🟢 CallResult]");
          debug!(msg_id, ?payload);
          return self.handle_call_result(&msg_id, &payload).await;
        }
        OcppMessageFrame::CallError {
          msg_id,
          error_code,
          description,
        } => {
          info!("🔌 [🔴 CallError] {}", error_code);
          debug!(msg_id, error_code, description);
          return self.handle_call_error(&msg_id).await;
        }
      }
    }

    anyhow::bail!("Invalid text message")
  }
}

impl MessageHandler {
  async fn handle_ocpp_request<Req, Res, F, Fut>(
    msg_id: &str,
    payload: Value,
    make_response: F,
  ) -> Result<Option<String>>
  where
    Req: DeserializeOwned,
    Res: Serialize + Debug,
    F: FnOnce(Req) -> Fut,
    Fut: Future<Output = Result<Res>>,
  {
    let request: Req = serde_json::from_value(payload)?;
    let response = make_response(request).await?;

    let ocpp_message = OcppMessageFrame::CallResult {
      msg_id: msg_id.to_string(),
      payload: serde_json::to_value(&response)?,
    };

    let response_string = serde_json::to_string(&ocpp_message.to_frame())?;

    info!("🔌 [🟢 CallResult]");
    debug!(msg_id, ?response);

    Ok(Some(response_string))
  }

  async fn handle_call(
    &mut self,
    msg_id: &str,
    action: &OcppAction,
    payload: &Value,
  ) -> Result<Option<String>> {
    use OcppAction::*;

    match action {
      GetConfiguration => {
        Self::handle_ocpp_request::<GetConfigurationRequest, GetConfigurationResponse, _, _>(
          msg_id,
          payload.clone(),
          |_request| async move {
            Ok(GetConfigurationResponse {
              ..Default::default()
            })
          },
        )
        .await
      }
      _ => anyhow::bail!("Unknown OCPP action to parse payload"),
    }
  }

  async fn handle_call_error(
    &self,
    msg_id: &str,
  ) -> Result<Option<String>> {
    use OcppAction::*;

    Ok(None)
  }

  async fn handle_call_result(
    &self,
    msg_id: &str,
    payload: &Value,
  ) -> Result<Option<String>> {
    use OcppAction::*;

    Ok(None)
  }
}
