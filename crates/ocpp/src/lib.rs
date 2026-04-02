#![allow(unused)]

use std::sync::Arc;

use common::{ChargePointConfig, Config, OcppVersion};

use crate::{
  msg_generator::{MessageGenerator, MessageGeneratorConfig, MessageGeneratorConfigBuilder},
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
  ocpp_version: &OcppVersion,
  config: &ChargePointConfig,
) -> (Box<dyn MessageGenerator>, Box<dyn MessageHandler>) {
  let mut config_builder = MessageGeneratorConfigBuilder::default();

  // Use first defined tag
  if let Some(id_tag) = config.id_tags.first() {
    config_builder = config_builder.id_tag(id_tag.clone());
  }

  let msg_generator_config = config_builder.build();

  match ocpp_version {
    #[cfg(feature = "ocpp1_6")]
    OcppVersion::V1_6 => {
      use crate::v1_6::{
        msg_generator::V16MessageGenerator, msg_handler::V16MessageHandler, types::OcppAction,
      };
      use common::SharedData;

      let shared_data = SharedData::<OcppAction>::default();
      (
        Box::new(V16MessageGenerator::new(
          msg_generator_config,
          shared_data.clone(),
        )),
        Box::new(V16MessageHandler::new(shared_data.clone())),
      )
    }

    #[cfg(feature = "ocpp2_0_1")]
    OcppVersion::V2_0_1 => (
      // Box::new(Ocpp201MessageGenerator::new(msg_generator_config)),
      // Box::new(Ocpp201MessageHandler::new()),
    ),

    #[cfg(feature = "ocpp2_1")]
    OcppVersion::V2_1 => (
      // Box::new(Ocpp21MessageGenerator::new(cmsg_generator_config)),
      // Box::new(Ocpp21MessageHandler::new()),
    ),

    _ => panic!("OCPP version not supported in this build"),
  }
}
