use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "messageTypeId", rename_all = "camelCase")]
pub enum OcppMessage {
  #[serde(rename_all = "camelCase")]
  Call {
    message_id: String,
    action: String,
    payload: Value,
  },
  #[serde(rename_all = "camelCase")]
  CallResult {
    message_id: String,
    payload: Value,
  },
  #[serde(rename_all = "camelCase")]
  CallError {
    message_id: String,
    error_code: String,
    error_description: String,
  },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OcppAction {
  // Core Profile
  Authorize,
  BootNotification,
  Heartbeat,
  MeterValues,
  StatusNotification,
  TransactionEvent,

  // Firmware Management
  GetBaseReport,
  GetReport,
  GetVariables,
  SetVariables,
  UpdateFirmware,
  PublishFirmware,
  UnpublishFirmware,
  GetFirmwareStatusNotification,
  FirmwareStatusNotification,

  // Security
  CertificateSigned,
  DeleteCertificate,
  GetInstalledCertificateIds,
  InstallCertificate,
  SecurityEventNotification,
  SignCertificate,

  // Smart Charging
  SetChargingProfile,
  ClearChargingProfile,
  GetChargingProfiles,
  GetCompositeSchedule,
  NotifyChargingLimit,
  ClearVariableMonitoring,
  SetVariableMonitoring,

  // Reservation
  ReserveNow,
  CancelReservation,

  // Remote Trigger
  TriggerMessage,

  // Remote Control
  ChangeAvailability,
  UnlockConnector,
  Reset,
  ClearCache,

  // ISO 15118 / Plug & Charge
  Get15118EVCertificate,
  GetCertificateStatus,
  GetCRL,

  // Local Auth List
  GetLocalListVersion,
  SendLocalList,
  SetNetworkProfile,

  // Custom / Extended
  DataTransfer,
}

impl FromStr for OcppAction {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use OcppAction::*;

    match s {
      // Core Profile
      "Authorize" => Ok(Authorize),
      "BootNotification" => Ok(BootNotification),
      "Heartbeat" => Ok(Heartbeat),
      "MeterValues" => Ok(MeterValues),
      "StatusNotification" => Ok(StatusNotification),
      "TransactionEvent" => Ok(TransactionEvent),

      // Firmware Management
      "GetBaseReport" => Ok(GetBaseReport),
      "GetReport" => Ok(GetReport),
      "GetVariables" => Ok(GetVariables),
      "SetVariables" => Ok(SetVariables),
      "UpdateFirmware" => Ok(UpdateFirmware),
      "PublishFirmware" => Ok(PublishFirmware),
      "UnpublishFirmware" => Ok(UnpublishFirmware),
      "GetFirmwareStatusNotification" => Ok(GetFirmwareStatusNotification),
      "FirmwareStatusNotification" => Ok(FirmwareStatusNotification),

      // Security
      "CertificateSigned" => Ok(CertificateSigned),
      "DeleteCertificate" => Ok(DeleteCertificate),
      "GetInstalledCertificateIds" => Ok(GetInstalledCertificateIds),
      "InstallCertificate" => Ok(InstallCertificate),
      "SecurityEventNotification" => Ok(SecurityEventNotification),
      "SignCertificate" => Ok(SignCertificate),

      // Smart Charging
      "SetChargingProfile" => Ok(SetChargingProfile),
      "ClearChargingProfile" => Ok(ClearChargingProfile),
      "GetChargingProfiles" => Ok(GetChargingProfiles),
      "GetCompositeSchedule" => Ok(GetCompositeSchedule),
      "NotifyChargingLimit" => Ok(NotifyChargingLimit),
      "ClearVariableMonitoring" => Ok(ClearVariableMonitoring),
      "SetVariableMonitoring" => Ok(SetVariableMonitoring),

      // Reservation
      "ReserveNow" => Ok(ReserveNow),
      "CancelReservation" => Ok(CancelReservation),

      // Remote Trigger
      "TriggerMessage" => Ok(TriggerMessage),

      // Remote Control
      "ChangeAvailability" => Ok(ChangeAvailability),
      "UnlockConnector" => Ok(UnlockConnector),
      "Reset" => Ok(Reset),
      "ClearCache" => Ok(ClearCache),

      // ISO 15118 / Plug & Charge
      "Get15118EVCertificate" => Ok(Get15118EVCertificate),
      "GetCertificateStatus" => Ok(GetCertificateStatus),
      "GetCRL" => Ok(GetCRL),

      // Local Auth List
      "GetLocalListVersion" => Ok(GetLocalListVersion),
      "SendLocalList" => Ok(SendLocalList),
      "SetNetworkProfile" => Ok(SetNetworkProfile),

      // Custom / Extended
      "DataTransfer" => Ok(DataTransfer),

      _ => Err("Unknown OCPP 2.0.1 action"),
    }
  }
}

impl std::fmt::Display for OcppAction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use OcppAction::*;

    let s = match self {
      // Core
      Authorize => "Authorize",
      BootNotification => "BootNotification",
      Heartbeat => "Heartbeat",
      MeterValues => "MeterValues",
      StatusNotification => "StatusNotification",
      TransactionEvent => "TransactionEvent",

      // Firmware
      GetBaseReport => "GetBaseReport",
      GetReport => "GetReport",
      GetVariables => "GetVariables",
      SetVariables => "SetVariables",
      UpdateFirmware => "UpdateFirmware",
      PublishFirmware => "PublishFirmware",
      UnpublishFirmware => "UnpublishFirmware",
      GetFirmwareStatusNotification => "GetFirmwareStatusNotification",
      FirmwareStatusNotification => "FirmwareStatusNotification",

      // Security
      CertificateSigned => "CertificateSigned",
      DeleteCertificate => "DeleteCertificate",
      GetInstalledCertificateIds => "GetInstalledCertificateIds",
      InstallCertificate => "InstallCertificate",
      SecurityEventNotification => "SecurityEventNotification",
      SignCertificate => "SignCertificate",

      // Smart Charging
      SetChargingProfile => "SetChargingProfile",
      ClearChargingProfile => "ClearChargingProfile",
      GetChargingProfiles => "GetChargingProfiles",
      GetCompositeSchedule => "GetCompositeSchedule",
      NotifyChargingLimit => "NotifyChargingLimit",
      ClearVariableMonitoring => "ClearVariableMonitoring",
      SetVariableMonitoring => "SetVariableMonitoring",

      // Reservation
      ReserveNow => "ReserveNow",
      CancelReservation => "CancelReservation",

      // Remote Trigger
      TriggerMessage => "TriggerMessage",

      // Remote Control
      ChangeAvailability => "ChangeAvailability",
      UnlockConnector => "UnlockConnector",
      Reset => "Reset",
      ClearCache => "ClearCache",

      // ISO 15118
      Get15118EVCertificate => "Get15118EVCertificate",
      GetCertificateStatus => "GetCertificateStatus",
      GetCRL => "GetCRL",

      // Local Auth List
      GetLocalListVersion => "GetLocalListVersion",
      SendLocalList => "SendLocalList",
      SetNetworkProfile => "SetNetworkProfile",

      // Custom / Extended
      DataTransfer => "DataTransfer",
    };

    write!(f, "{s}")
  }
}
