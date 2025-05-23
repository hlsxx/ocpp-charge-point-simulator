use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum OcppVersion {
  V1_6,
  V2_0_1,
  V2_1,
}

impl Display for OcppVersion {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let version_str = match self {
      OcppVersion::V1_6 => "ocpp1.6",
      OcppVersion::V2_0_1 => "ocpp2.0.1",
      OcppVersion::V2_1 => "ocpp2.1",
    };

    write!(f, "{}", version_str)
  }
}

impl OcppVersion {
  pub const HEADER_V1_6: &'static str = "ocpp1.6";
  pub const HEADER_V2_0_1: &'static str = "ocpp2.0.1";
  pub const HEADER_V2_1: &'static str = "ocpp2.1";

  pub fn from_header_value(header: &str) -> Option<Self> {
    match header {
      Self::HEADER_V1_6 => Some(OcppVersion::V1_6),
      Self::HEADER_V2_0_1 => Some(OcppVersion::V2_0_1),
      Self::HEADER_V2_1 => Some(OcppVersion::V2_1),
      _ => None,
    }
  }
}
