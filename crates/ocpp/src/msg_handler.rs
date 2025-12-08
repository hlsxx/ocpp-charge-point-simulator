use anyhow::Result;
use async_trait::async_trait;
use common::shared_data::SharedDataValue;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};

use super::{
  v1_6::types::OcppAction as V16OcppAction, v2_0_1::types::OcppAction as V201OcppAction,
  v2_1::types::OcppAction as V21OcppAction,
};

#[derive(Debug, Serialize, Clone)]
pub enum MessageFrame<A> {
  Call {
    msg_id: String,
    action: A,
    payload: Value,
  },
  CallResult {
    msg_id: String,
    payload: Value,
  },
  CallError {
    msg_id: String,
    error_code: String,
    description: String,
  },
}

impl<A: Serialize> MessageFrame<A> {
  pub fn to_frame(&self) -> Value {
    match self {
      MessageFrame::Call {
        msg_id,
        action,
        payload,
      } => {
        json!([2, msg_id, action, payload])
      }
      MessageFrame::CallResult { msg_id, payload } => {
        json!([3, msg_id, payload])
      }
      MessageFrame::CallError {
        msg_id,
        error_code,
        description,
      } => {
        json!([4, msg_id, error_code, description])
      }
    }
  }
}

#[derive(Debug, Clone)]
pub enum MessageFrameType {
  V1_6(MessageFrame<V16OcppAction>),
  V2_0_1(MessageFrame<V201OcppAction>),
  V2_1(MessageFrame<V21OcppAction>),
}

impl MessageFrameType {
  pub fn to_frame(&self) -> Value {
    match self {
      Self::V1_6(msg_frame) => msg_frame.to_frame(),
      Self::V2_0_1(msg_frame) => msg_frame.to_frame(),
      Self::V2_1(msg_frame) => msg_frame.to_frame(),
    }
  }
}

#[async_trait]
pub trait MessageHandler: SharedDataValue {
  async fn handle_text_message(&mut self, text: &str) -> Result<Option<String>>;

  fn parse_ocpp_message(&self, text: &str) -> Result<MessageFrameType>;
}
