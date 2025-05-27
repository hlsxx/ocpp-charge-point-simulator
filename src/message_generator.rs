use serde::Serialize;
use serde_json::Value;

pub trait MessageGeneratorTrait {
  type OcppAction;

  type BootNotification;
  type Heartbeat;
  type Authorize;
  type StartTransaction;
  type StopTransaction;
  type StatusNotification;
  type MeterValues;
  type DiagnosticsStatusNotification;
  type FirmwareStatusNotification;
  type DataTransfer;

  fn boot_notification(&self) -> Self::BootNotification;
  fn heartbeat(&self) -> Self::Heartbeat;
  fn authorize(&self) -> Self::Authorize;
  fn start_transaction(&self) -> Self::StartTransaction;
  fn stop_transaction(&self) -> Self::StopTransaction;
  fn status_notification(&self) -> Self::StatusNotification;
  fn meter_values(&self) -> Self::MeterValues;
  fn diagnostics_status_notification(&self) -> Self::DiagnosticsStatusNotification;
  fn firmware_status_notification(&self) -> Self::FirmwareStatusNotification;
  fn data_transfer(&self) -> Self::DataTransfer;

  fn next_id(&self) -> String;
  fn to_frame<T: Serialize>(&self, action: Self::OcppAction, payload: T) -> Value;
}
