#![allow(unused)]

use common::OcppVersion;

use crate::{
  msg_generator::{MessageGenerator, MessageGeneratorConfig},
  msg_handler::MessageHandler,
};

pub mod mock_data;
pub mod msg_generator;
pub mod msg_handler;
pub mod types;
pub mod v1_6;
pub mod v2_0_1;
pub mod v2_1;

pub fn create_ocpp_handlers(
  ocpp_version: OcppVersion,
) -> (Box<dyn MessageGenerator>, Box<dyn MessageHandler>) {
  let msg_generator_config = MessageGeneratorConfig::default();

  match ocpp_version {
    #[cfg(feature = "ocpp1_6")]
    OcppVersion::V1_6 => {
      let shared_data = SharedData::<ocpp::v1_6::types::OcppAction>::default();
      (
        Box::new(Ocpp16MessageGenerator::new(
          msg_generator_config,
          shared_data.clone(),
        )),
        Box::new(Ocpp16MessageHandler::new(shared_data.clone())),
      )
    }

    #[cfg(feature = "ocpp2_0_1")]
    OcppVersion::V2_0_1 => (
      Box::new(Ocpp201MessageGenerator::new(msg_generator_config)),
      Box::new(Ocpp201MessageHandler::new()),
    ),

    #[cfg(feature = "ocpp2_1")]
    OcppVersion::V2_1 => (
      Box::new(Ocpp21MessageGenerator::new(cmsg_generator_config)),
      Box::new(Ocpp21MessageHandler::new()),
    ),

    _ => panic!("OCPP version not supported in this build"),
  }
}
