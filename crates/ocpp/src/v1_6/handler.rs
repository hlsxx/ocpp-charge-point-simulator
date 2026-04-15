use std::{fmt::Debug, str::FromStr};

use super::types::OcppAction;
use crate::{
  handler::{MessageFrame, MessageFrameType, MessageHandler},
  types::CommonOcppResponse,
};
use anyhow::Result;
use async_trait::async_trait;
use common::SharedData;
use rust_ocpp::v1_6::messages::{
  authorize::AuthorizeResponse,
  change_configuration::ChangeConfigurationRequest,
  get_configuration::{GetConfigurationRequest, GetConfigurationResponse},
  heart_beat::HeartbeatRequest,
  remote_start_transaction::RemoteStartTransactionRequest,
  start_transaction::StartTransactionResponse,
};

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use tracing::{debug, info};

pub struct V16MessageHandler {
  shared_data: SharedData<OcppAction>,
}

impl V16MessageHandler {
  pub fn new(shared_data: SharedData<OcppAction>) -> Self {
    Self { shared_data }
  }
}

#[async_trait]
impl MessageHandler for V16MessageHandler {
  fn parse_raw_ocpp_msg(&self, msg: &str) -> Result<MessageFrameType> {
    let arr: Vec<Value> = serde_json::from_str(msg)?;

    match arr.first().and_then(|v| v.as_u64()) {
      Some(2) => {
        let msg_id = arr[1].as_str().unwrap_or("").to_string();
        let action_string = arr[2].as_str().unwrap_or("").to_string();
        let payload = arr[3].clone();

        let action = OcppAction::from_str(action_string.as_str())
          .map_err(|err| anyhow::anyhow!("Invalid OCPP action: {}", err))?;

        info!("[🔵 Call] {}", action);

        Ok(MessageFrameType::V1_6(MessageFrame::Call {
          msg_id,
          action,
          payload,
        }))
      }
      Some(3) => {
        let msg_id = arr[1].as_str().unwrap_or("").to_string();
        let payload = arr[2].clone();

        info!("[🟢 CallResult]");

        Ok(MessageFrameType::V1_6(MessageFrame::CallResult {
          msg_id,
          payload,
        }))
      }
      Some(4) => {
        let msg_id = arr[1].as_str().unwrap_or("").to_string();
        let error_code = arr[2].as_str().unwrap_or("").to_string();
        let description = arr[3].as_str().unwrap_or("").to_string();

        info!("[🔴 CallError] {}", error_code);

        Ok(MessageFrameType::V1_6(MessageFrame::CallError {
          msg_id,
          error_code,
          description,
        }))
      }
      _ => anyhow::bail!("Unknown OCPP v1.6 message type"),
    }
  }

  async fn handle_text_message(&mut self, text: &str) -> Result<Option<String>> {
    if let MessageFrameType::V1_6(ocpp_message) = self.parse_raw_ocpp_msg(text)? {
      match ocpp_message {
        MessageFrame::Call {
          msg_id,
          action,
          payload,
        } => {
          info!("[🔵 Call] {}", action);
          debug!(?action, msg_id, ?payload);
          return self.handle_call(&msg_id, &action, &payload).await;
        }
        MessageFrame::CallResult { msg_id, payload } => {
          info!("[🟢 CallResult]");
          debug!(msg_id, ?payload);
          self.handle_call_result(&msg_id, &payload).await?;
          return Ok(None);
        }
        MessageFrame::CallError {
          msg_id,
          error_code,
          description,
        } => {
          info!("[🔴 CallError] {}", error_code);
          debug!(msg_id, error_code, description);
          return self.handle_call_error(&msg_id).await;
        }
      }
    }

    anyhow::bail!("Invalid text message")
  }

  async fn handle_call_result(
    &self,
    msg_id: &str,
    payload: &Value,
  ) -> Result<Option<CommonOcppResponse>> {
    let ocpp_action = self.shared_data.get_msg(msg_id).await;
    match ocpp_action {
      Some(ocpp_action) => match ocpp_action {
        OcppAction::StartTransaction => {
          let res: StartTransactionResponse = serde_json::from_value(payload.clone())?;

          self
            .shared_data
            .write(|data| data.transaction_id = Some(res.transaction_id))
            .await;

          Ok(Some(CommonOcppResponse::StartTransaction {
            transaction_id: res.transaction_id,
          }))
        }
        OcppAction::Authorize => {
          let res: AuthorizeResponse = serde_json::from_value(payload.clone())?;
          Ok(Some(CommonOcppResponse::Authorize {
            status: res.id_tag_info.status.into(),
          }))
        }
        _ => Ok(None),
      },
      None => anyhow::bail!("msg_id not found"),
    }
  }
}

impl V16MessageHandler {
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

  async fn handle_call_error(&self, _msg_id: &str) -> Result<Option<String>> {
    Ok(None)
  }

  pub fn parse_payload<T: DeserializeOwned>(value: serde_json::Value) -> Result<T> {
    let payload: T = serde_json::from_value(value)?;
    Ok(payload)
  }

  pub fn parse_change_configuration_payload(
    payload: serde_json::Value,
  ) -> Result<ChangeConfigurationRequest> {
    V16MessageHandler::parse_payload::<ChangeConfigurationRequest>(payload)
  }

  pub fn parse_remote_start_transaction_payload(
    payload: serde_json::Value,
  ) -> Result<RemoteStartTransactionRequest> {
    V16MessageHandler::parse_payload::<RemoteStartTransactionRequest>(payload)
  }

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

    let ocpp_message = MessageFrame::<OcppAction>::CallResult {
      msg_id: msg_id.to_string(),
      payload: serde_json::to_value(&response)?,
    };

    let response_string = serde_json::to_string(&ocpp_message.to_frame())?;

    info!("[🟢 CallResult]");
    debug!(msg_id, ?response);

    Ok(Some(response_string))
  }
}
