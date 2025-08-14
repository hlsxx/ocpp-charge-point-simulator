use rust_ocpp::{
  v1_6::types::ChargePointStatus,
  v2_0_1::enumerations::connector_status_enum_type::ConnectorStatusEnumType,
};

#[derive(Debug, Clone)]
pub enum CommonConnectorStatusType {
  Available,
  /// When a Connector becomes no longer available for a new user but there is no ongoing Transaction (yet). Typically a Connector is in preparing state when a user presents a tag, inserts a cable or a vehicle occupies the parking bay 6 (Operative)
  Preparing,
  /// When the contactor of a Connector closes, allowing the vehicle to charge (Operative)
  Charging,
  /// When the EV is connected to the EVSE but the EVSE is not offering energy to the EV, e.g. due to a smart charging restriction, local supply power constraints, or as the result of StartTransaction.conf indicating that charging is not allowed etc. (Operative)
  SuspendedEVSE,
  /// When the EV is connected to the EVSE and the EVSE is offering energy but the EV is not taking any energy. (Operative)
  SuspendedEV,
  /// When a Transaction has stopped at a Connector, but the Connector is not yet available for a new user, e.g. the cable has not been removed or the vehicle has not left the parking bay (Operative)
  Finishing,
  /// When a Connector becomes reserved as a result of a Reserve Now command (Operative)
  Reserved,
  /// When a Connector becomes unavailable as the result of a Change Availability command or an event upon which the Charge Point transitions to unavailable at its discretion. Upon receipt of a Change Availability command, the status MAY change immediately or the change MAY be scheduled. When scheduled, the Status Notification shall be send when the availability change becomes effective (Inoperative).
  Unavailable,
  /// When a Charge Point or connector has reported an error and is not available for energy delivery, (Inoperative)
  Faulted,
}

impl From<CommonConnectorStatusType> for ChargePointStatus {
  fn from(value: CommonConnectorStatusType) -> Self {
    match value {
      CommonConnectorStatusType::Available => ChargePointStatus::Available,
      CommonConnectorStatusType::Preparing => ChargePointStatus::Preparing,
      CommonConnectorStatusType::Charging => ChargePointStatus::Charging,
      CommonConnectorStatusType::SuspendedEVSE => ChargePointStatus::SuspendedEVSE,
      CommonConnectorStatusType::SuspendedEV => ChargePointStatus::SuspendedEV,
      CommonConnectorStatusType::Finishing => ChargePointStatus::Finishing,
      CommonConnectorStatusType::Reserved => ChargePointStatus::Reserved,
      CommonConnectorStatusType::Unavailable => ChargePointStatus::Unavailable,
      CommonConnectorStatusType::Faulted => ChargePointStatus::Faulted,
    }
  }
}

impl From<CommonConnectorStatusType> for ConnectorStatusEnumType {
  fn from(value: CommonConnectorStatusType) -> Self {
    match value {
      CommonConnectorStatusType::Available => ConnectorStatusEnumType::Available,
      CommonConnectorStatusType::Preparing => ConnectorStatusEnumType::Available,
      CommonConnectorStatusType::Charging => ConnectorStatusEnumType::Occupied,
      CommonConnectorStatusType::SuspendedEVSE => ConnectorStatusEnumType::Occupied,
      CommonConnectorStatusType::SuspendedEV => ConnectorStatusEnumType::Occupied,
      CommonConnectorStatusType::Finishing => ConnectorStatusEnumType::Occupied,
      CommonConnectorStatusType::Reserved => ConnectorStatusEnumType::Reserved,
      CommonConnectorStatusType::Unavailable => ConnectorStatusEnumType::Unavailable,
      CommonConnectorStatusType::Faulted => ConnectorStatusEnumType::Faulted,
    }
  }
}
