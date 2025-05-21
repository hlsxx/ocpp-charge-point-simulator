use serde::Serialize;
use serde_json::Value;

pub trait MessageGenerator {
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

  fn boot_notification() -> Self::BootNotification;
  fn heartbeat() -> Self::Heartbeat;
  fn authorize() -> Self::Authorize;
  fn start_transaction() -> Self::StartTransaction;
  fn stop_transaction() -> Self::StopTransaction;
  fn status_notification() -> Self::StatusNotification;
  fn meter_values() -> Self::MeterValues;
  fn diagnostics_status_notification() -> Self::DiagnosticsStatusNotification;
  fn firmware_status_notification() -> Self::FirmwareStatusNotification;
  fn data_transfer() -> Self::DataTransfer;

  fn to_frame<T: Serialize>(action: Self::OcppAction, payload: T) -> Value;
}
