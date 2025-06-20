use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
// #[serde(untagged)]
pub enum OcppMessage {
  Call {
    msg_id: String,
    action: OcppAction,
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


#[derive(Debug, Serialize, Deserialize)]
pub enum OcppAction {
  // CP → CSMS
  BootNotification,
  Heartbeat,
  Authorize,
  StartTransaction,
  StopTransaction,
  StatusNotification,
  MeterValues,
  DiagnosticsStatusNotification,
  FirmwareStatusNotification,
  DataTransfer,

  // CSMS → CP
  RemoteStartTransaction,
  RemoteStopTransaction,
  Reset,
  ChangeAvailability,
  ChangeConfiguration,
  GetConfiguration,
  ClearCache,
  UpdateFirmware,
  GetDiagnostics,
  UnlockConnector,
  CancelReservation,
  ReserveNow,
  SetChargingProfile,
  ClearChargingProfile,
  GetCompositeSchedule,
  GetLocalListVersion,
  SendLocalList,
}

impl FromStr for OcppAction {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use OcppAction::*;

    match s {
      "BootNotification" => Ok(BootNotification),
      "Heartbeat" => Ok(Heartbeat),
      "Authorize" => Ok(Authorize),
      "StartTransaction" => Ok(StartTransaction),
      "StopTransaction" => Ok(StopTransaction),
      "StatusNotification" => Ok(StatusNotification),
      "MeterValues" => Ok(MeterValues),
      "DiagnosticsStatusNotification" => Ok(DiagnosticsStatusNotification),
      "FirmwareStatusNotification" => Ok(FirmwareStatusNotification),
      "DataTransfer" => Ok(DataTransfer),

      "RemoteStartTransaction" => Ok(RemoteStartTransaction),
      "RemoteStopTransaction" => Ok(RemoteStopTransaction),
      "Reset" => Ok(Reset),
      "ChangeAvailability" => Ok(ChangeAvailability),
      "ChangeConfiguration" => Ok(ChangeConfiguration),
      "GetConfiguration" => Ok(GetConfiguration),
      "ClearCache" => Ok(ClearCache),
      "UpdateFirmware" => Ok(UpdateFirmware),
      "GetDiagnostics" => Ok(GetDiagnostics),
      "UnlockConnector" => Ok(UnlockConnector),
      "CancelReservation" => Ok(CancelReservation),
      "ReserveNow" => Ok(ReserveNow),
      "SetChargingProfile" => Ok(SetChargingProfile),
      "ClearChargingProfile" => Ok(ClearChargingProfile),
      "GetCompositeSchedule" => Ok(GetCompositeSchedule),
      "GetLocalListVersion" => Ok(GetLocalListVersion),
      "SendLocalList" => Ok(SendLocalList),

      _ => Err("Unknown OCPP v1.6 action"),
    }
  }
}

impl std::fmt::Display for OcppAction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use OcppAction::*;

    let s = match self {
      BootNotification => "BootNotification",
      Heartbeat => "Heartbeat",
      Authorize => "Authorize",
      StartTransaction => "StartTransaction",
      StopTransaction => "StopTransaction",
      StatusNotification => "StatusNotification",
      MeterValues => "MeterValues",
      DiagnosticsStatusNotification => "DiagnosticsStatusNotification",
      FirmwareStatusNotification => "FirmwareStatusNotification",
      DataTransfer => "DataTransfer",
      RemoteStartTransaction => "RemoteStartTransaction",
      RemoteStopTransaction => "RemoteStopTransaction",
      Reset => "Reset",
      ChangeAvailability => "ChangeAvailability",
      ChangeConfiguration => "ChangeConfiguration",
      GetConfiguration => "GetConfiguration",
      ClearCache => "ClearCache",
      UpdateFirmware => "UpdateFirmware",
      GetDiagnostics => "GetDiagnostics",
      UnlockConnector => "UnlockConnector",
      CancelReservation => "CancelReservation",
      ReserveNow => "ReserveNow",
      SetChargingProfile => "SetChargingProfile",
      ClearChargingProfile => "ClearChargingProfile",
      GetCompositeSchedule => "GetCompositeSchedule",
      GetLocalListVersion => "GetLocalListVersion",
      SendLocalList => "SendLocalList",
    };

    write!(f, "{s}")
  }
}
