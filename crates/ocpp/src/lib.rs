pub mod mock_data;
pub mod msg_generator;
pub mod msg_handler;
pub mod types;
pub mod v1_6;
pub mod v2_0_1;
pub mod v2_1;

use crate::{msg_generator::MessageGenerator, msg_handler::MessageHandler};
use common::{ChargePointConfig, OcppVersion, SharedData};

pub struct OcppSession {
  pub generator: Box<dyn MessageGenerator>,
  pub handler: Box<dyn MessageHandler>,
}

impl OcppSession {
  pub async fn new(ocpp_version: &OcppVersion, config: ChargePointConfig) -> Self {
    match ocpp_version {
      #[cfg(feature = "ocpp1_6")]
      OcppVersion::V1_6 => {
        use crate::v1_6::{
          msg_generator::V16MessageGenerator, msg_handler::V16MessageHandler, types::OcppAction,
        };

        let shared_data = SharedData::<OcppAction>::from_cp_config(&config).await;

        Self {
          generator: Box::new(V16MessageGenerator::new(config, shared_data.clone())),
          handler: Box::new(V16MessageHandler::new(shared_data)),
        }
      }
      _ => panic!("OCPP version not supported in this build"),
    }
  }
}
