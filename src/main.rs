pub mod env;
pub mod message_generator;
pub mod ocpp;
pub mod simulator;
pub mod v1_6;
pub mod ws_client;
pub mod config;

use anyhow::Result;
use env::Env;
use ocpp::OcppVersion;
use simulator::{Simulator, SimulatorConfigBuilder};
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {
  let env = Env::try_load()?;

  tracing_subscriber::fmt()
    .with_max_level(if env.debug_mode {
      Level::DEBUG
    } else {
      Level::INFO
    })
    .with_target(true)
    .init();

  let simulator_config = SimulatorConfigBuilder::new()
    .csms_url(env.csms_url)
    .ocpp_version(OcppVersion::from(&env.charge_point_ocpp_version).unwrap())
    .clients_num(env.charge_point_clients_num)
    .build();

  Simulator::new(simulator_config).run().await?;

  Ok(())
}
