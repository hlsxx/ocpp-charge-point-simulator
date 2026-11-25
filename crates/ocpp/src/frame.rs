use anyhow::{Result, bail};
use serde_json::Value;

/// A protocol-version-independent OCPP WebSocket frame.
///
/// Payload decoding is intentionally left to the version-specific handler.
#[derive(Debug)]
pub enum ParsedFrame {
  Call {
    msg_id: String,
    action: String,
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

pub struct Frame;

impl Frame {
  pub fn parse(text: &str) -> Result<ParsedFrame> {
    let frame: Vec<Value> = serde_json::from_str(text)?;
    let message_type = frame
      .first()
      .and_then(Value::as_u64)
      .ok_or_else(|| anyhow::anyhow!("missing OCPP message type"))?;

    match message_type {
      2 => {
        if frame.len() != 4 {
          bail!("CALL frame must contain 4 elements, got {}", frame.len());
        }

        Ok(ParsedFrame::Call {
          msg_id: required_string(&frame[1], "message id")?,
          action: required_string(&frame[2], "action")?,
          payload: frame[3].clone(),
        })
      }
      3 => {
        if frame.len() != 3 {
          bail!(
            "CALLRESULT frame must contain 3 elements, got {}",
            frame.len()
          );
        }

        Ok(ParsedFrame::CallResult {
          msg_id: required_string(&frame[1], "message id")?,
          payload: frame[2].clone(),
        })
      }
      4 => {
        if frame.len() < 4 {
          bail!(
            "CALLERROR frame must contain at least 4 elements, got {}",
            frame.len()
          );
        }

        Ok(ParsedFrame::CallError {
          msg_id: required_string(&frame[1], "message id")?,
          error_code: required_string(&frame[2], "error code")?,
          description: required_string(&frame[3], "description")?,
        })
      }
      other => bail!("unknown OCPP message type: {other}"),
    }
  }
}

fn required_string(value: &Value, field: &str) -> Result<String> {
  value
    .as_str()
    .map(str::to_owned)
    .ok_or_else(|| anyhow::anyhow!("{field} must be a string"))
}

#[cfg(test)]
mod tests {
  use super::{Frame, ParsedFrame};

  #[test]
  fn parses_call() {
    let parsed = Frame::parse(r#"[2,"123","Heartbeat",{}]"#).unwrap();

    assert!(matches!(
      parsed,
      ParsedFrame::Call {
        msg_id,
        action,
        ..
      } if msg_id == "123" && action == "Heartbeat"
    ));
  }

  #[test]
  fn rejects_short_call() {
    assert!(Frame::parse(r#"[2,"123","Heartbeat"]"#).is_err());
  }

  #[test]
  fn rejects_non_string_message_id() {
    assert!(Frame::parse(r#"[3,123,{}]"#).is_err());
  }
}
