use anyhow::{Context, Result};
use dotenvy::dotenv;
use std::env::var;

pub struct Env {
  pub debug_mode: bool,

  // WebSocket
  pub ws_host: String,
  pub ws_port: u16,

  // PostgreSQL
  pub database_user: String,
  pub database_password: Option<String>,
  pub database_host: String,
  pub database_port: u32,
  pub database_name: String,
}

impl Env {
  pub fn try_load() -> Result<Self> {
    dotenv().context("Failed to load .env file")?;

    // Server env
    let debug_mode = var("DEBUG_MODE")
      .context("Missing DEBUG_MODE")?
      .parse::<bool>()
      .context("DEBUG_MODE must be a boolean (true/false)")?;

    // WebSocket env
    let ws_host = var("WS_HOST").context("Missing WS_HOST")?;
    let ws_port = var("WS_PORT")
      .context("Missing WS_PORT")?
      .parse::<u16>()
      .context("WS_PORT must be a valid port number (u16)")?;

    // Database env
    let database_user = var("DATABASE_USER").context("Missing DATABASE_USER")?;
    let database_password = var("DATABASE_PASSWORD").ok();
    let database_host = var("DATABASE_HOST").context("Missing DATABASE_HOST")?;
    let database_port = var("DATABASE_PORT")
      .context("Missing DATABASE_PORT")?
      .parse::<u32>()
      .context("DATABASE_PORT must be a valid port number (u32)")?;
    let database_name = var("DATABASE_NAME").context("Missing DATABASE_NAME")?;

    Ok(Self {
      debug_mode,
      ws_host,
      ws_port,

      database_user,
      database_password,
      database_host,
      database_port,
      database_name,
    })
  }
}
