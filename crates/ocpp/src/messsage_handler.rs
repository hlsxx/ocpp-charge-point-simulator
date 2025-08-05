use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;
use serde_json::{json, Value};

use super::{
  v1_6::types::OcppAction as V16OcppAction, v2_0_1::types::OcppAction as V201OcppAction,
  v2_1::types::OcppAction as V21OcppAction,
};

#[derive(Debug, Serialize, Clone)]
pub enum OcppMessageFrame<A> {
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

impl<A: Serialize> OcppMessageFrame<A> {
  pub fn to_frame(&self) -> Value {
    match self {
      OcppMessageFrame::Call {
        msg_id,
        action,
        payload,
      } => {
        json!([2, msg_id, action, payload])
      }
      OcppMessageFrame::CallResult { msg_id, payload } => {
        json!([3, msg_id, payload])
      }
      OcppMessageFrame::CallError {
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
pub enum OcppMessageFrameType {
  V1_6(OcppMessageFrame<V16OcppAction>),
  V2_0_1(OcppMessageFrame<V201OcppAction>),
  V2_1(OcppMessageFrame<V21OcppAction>),
}

impl OcppMessageFrameType {
  pub fn to_frame(&self) -> Value {
    match self {
      Self::V1_6(msg_frame) => msg_frame.to_frame(),
      Self::V2_0_1(msg_frame) => msg_frame.to_frame(),
      Self::V2_1(msg_frame) => msg_frame.to_frame(),
    }
  }
}

#[async_trait]
pub trait OcppMessageHandler: Send + Sync {
  async fn handle_text_message(&mut self, text: &str) -> Result<Option<String>>;
  fn parse_ocpp_message(&self, text: &str) -> Result<OcppMessageFrameType>;
}
