use std::{fmt::Display, str::FromStr};

use rust_ocpp::v1_6::types::{Location, Measurand, Phase, ReadingContext, UnitOfMeasure, ValueFormat};
use serde::{Deserialize, Serialize};

pub struct DisplayMeasurand(pub Measurand);
pub struct DisplayUnitOfMeasure(pub UnitOfMeasure);
pub struct DisplayReadingContext(pub ReadingContext);
pub struct DisplayPhase(pub Phase);
pub struct DisplayLocation(pub Location);
pub struct DisplayValueFormat(pub ValueFormat);

use anyhow::{anyhow, Result};
use serde_json::{Value, json};

#[derive(Debug, Serialize, Deserialize)]
pub enum OcppMessageFrame {
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

impl OcppMessageFrame {
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

impl Display for DisplayMeasurand {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.0 {
      Measurand::CurrentExport => write!(f, "Current.Export"),
      Measurand::CurrentImport => write!(f, "Current.Import"),
      Measurand::CurrentOffered => write!(f, "Current.Offered"),
      Measurand::EnergyActiveExportRegister => write!(f, "Energy.Active.Export.Register"),
      Measurand::EnergyActiveImportRegister => write!(f, "Energy.Active.Import.Register"),
      Measurand::EnergyReactiveExportRegister => {
        write!(f, "Energy.Reactive.Export.Register")
      }
      Measurand::EnergyReactiveImportRegister => {
        write!(f, "Energy.Reactive.Import.Register")
      }
      Measurand::EnergyActiveExportInterval => write!(f, "Energy.Active.Export.Interval"),
      Measurand::EnergyActiveImportInterval => write!(f, "Energy.Active.Import.Interval"),
      Measurand::EnergyReactiveExportInterval => {
        write!(f, "Energy.Reactive.Export.Interval")
      }
      Measurand::EnergyReactiveImportInterval => {
        write!(f, "Energy.Reactive.Import.Interval")
      }
      Measurand::Frequency => write!(f, "Frequency"),
      Measurand::PowerActiveExport => write!(f, "Power.Active.Export"),
      Measurand::PowerActiveImport => write!(f, "Power.Active.Import"),
      Measurand::PowerFactor => write!(f, "Power.Factor"),
      Measurand::PowerOffered => write!(f, "Power.Offered"),
      Measurand::PowerReactiveExport => write!(f, "Power.Reactive.Export"),
      Measurand::PowerReactiveImport => write!(f, "Power.Reactive.Import"),
      Measurand::Rpm => write!(f, "RPM"),
      Measurand::SoC => write!(f, "SoC"),
      Measurand::Temperature => write!(f, "Temperature"),
      Measurand::Voltage => write!(f, "Voltage"),
    }
  }
}

impl Display for DisplayUnitOfMeasure {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.0 {
      UnitOfMeasure::Wh => write!(f, "Wh"),
      UnitOfMeasure::KWh => write!(f, "kWh"),
      UnitOfMeasure::Varh => write!(f, "varh"),
      UnitOfMeasure::Kvarh => write!(f, "kvarh"),
      UnitOfMeasure::W => write!(f, "W"),
      UnitOfMeasure::Kw => write!(f, "kW"),
      UnitOfMeasure::Va => write!(f, "VA"),
      UnitOfMeasure::Kva => write!(f, "kVA"),
      UnitOfMeasure::Var => write!(f, "var"),
      UnitOfMeasure::Kvar => write!(f, "kvar"),
      UnitOfMeasure::A => write!(f, "A"),
      UnitOfMeasure::V => write!(f, "V"),
      UnitOfMeasure::Celsius => write!(f, "Celsius"),
      UnitOfMeasure::Fahrenheit => write!(f, "Fahrenheit"),
      UnitOfMeasure::K => write!(f, "K"),
      UnitOfMeasure::Percent => write!(f, "Percent")
    }
  }
}

impl Display for DisplayReadingContext {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.0 {
      ReadingContext::InterruptionBegin => write!(f, "Interruption.Begin"),
      ReadingContext::InterruptionEnd => write!(f, "Interruption.End"),
      ReadingContext::Other => write!(f, "Other"),
      ReadingContext::SampleClock => write!(f, "Sample.Clock"),
      ReadingContext::SamplePeriodic => write!(f, "Sample.Periodic"),
      ReadingContext::TransactionBegin => write!(f, "Transaction.Begin"),
      ReadingContext::TransactionEnd => write!(f, "Transaction.End"),
      ReadingContext::Trigger => write!(f, "Trigger"),
    }
  }
}

impl Display for DisplayPhase {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.0 {
      Phase::L1 => write!(f, "L1"),
      Phase::L2 => write!(f, "L2"),
      Phase::L3 => write!(f, "L3"),
      Phase::N => write!(f, "N"),
      Phase::L1N => write!(f, "L1-N"),
      Phase::L2N => write!(f, "L2-N"),
      Phase::L3N => write!(f, "L3-N"),
      Phase::L1L2 => write!(f, "L1-L2"),
      Phase::L2L3 => write!(f, "L2-L3"),
      Phase::L3L1 => write!(f, "L3-L1"),
    }
  }
}

impl Display for DisplayLocation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.0 {
      Location::Body => write!(f, "Body"),
      Location::Cable => write!(f, "Cable"),
      Location::Ev => write!(f, "EV"),
      Location::Inlet => write!(f, "Inlet"),
      Location::Outlet => write!(f, "Outlet")
    }
  }
}

impl Display for DisplayValueFormat {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.0 {
      ValueFormat::Raw => write!(f, "Raw"),
      ValueFormat::SignedData => write!(f, "SignedData")
    }
  }
}
