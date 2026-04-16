use std::{
  collections::{HashMap, HashSet},
  sync::Arc,
};

use tokio::{sync::RwLock, time::Instant};

use crate::ChargePointConfig;

type MsgId = String;
type TagId = String;

pub trait SharedDataValue: Send + Sync {}
impl<A: Send + Sync> SharedDataValue for A {}

#[derive(Debug, Clone)]
pub struct ChargePointSettings {
  // 🔌 Core / Timing
  pub heartbeat_interval: u32,
  pub connection_timeout: u32,
  pub reset_retries: u32,

  // ⚡ Metering
  pub meter_value_sample_interval: u32,
  pub clock_aligned_data_interval: u32,
  pub meter_values_sampled_data: HashSet<String>,
  pub meter_values_aligned_data: String,
  pub stop_txn_sampled_data: String,
  pub stop_txn_aligned_data: String,

  // 🔄 Transaction behavior
  pub authorize_remote_tx_requests: bool,
  pub stop_transaction_on_ev_side_disconnect: bool,
  pub stop_transaction_on_invalid_id: bool,
  pub max_energy_on_invalid_id: u32,

  pub transaction_message_attempts: u32,
  pub transaction_message_retry_interval: u32,

  // 🔐 Authorization
  pub local_authorize_offline: bool,
  pub local_pre_authorize: bool,
  pub authorization_cache_enabled: bool,
  pub allow_offline_tx_for_unknown_id: bool,

  // 💳 Local Auth List
  pub local_auth_list_enabled: bool,
  pub local_auth_list_version: i32,
  pub send_local_list_max_length: u32,
  pub local_auth_list_max_length: u32,

  // 🔌 Connector / Hardware
  pub number_of_connectors: u32,
  pub connector_phase_rotation: String,

  // 🌐 Network / WebSocket
  pub websocket_ping_interval: u32,

  // ⚡ Smart Charging
  pub charge_profile_max_stack_level: u32,
  pub charging_schedule_allowed_charging_rate_unit: String,
  pub charging_schedule_max_periods: u32,

  // 📊 Limits / Misc
  pub get_configuration_max_keys: u32,
}

impl Default for ChargePointSettings {
  fn default() -> Self {
    Self {
      // 🔌 Core / Timing
      heartbeat_interval: 60,
      connection_timeout: 60,
      reset_retries: 3,

      // ⚡ Metering
      meter_value_sample_interval: 60,
      clock_aligned_data_interval: 0,
      meter_values_sampled_data: HashSet::from([
        "Energy.Active.Import.Register".to_string(),
        "Power.Active.Import".to_string(),
        "Current.Import".to_string(),
        "Voltage".to_string(),
        "Power.Factor".to_string(),
        "SoC".to_string(),
        "Frequency".to_string(),
        "Temperature".to_string(),
      ]),
      meter_values_aligned_data: "".to_string(),
      stop_txn_sampled_data: "Energy.Active.Import.Register".to_string(),
      stop_txn_aligned_data: "".to_string(),

      // 🔄 Transaction behavior
      authorize_remote_tx_requests: true,
      stop_transaction_on_ev_side_disconnect: true,
      stop_transaction_on_invalid_id: false,
      max_energy_on_invalid_id: 0,
      transaction_message_attempts: 3,
      transaction_message_retry_interval: 60,

      // 🔐 Authorization
      local_authorize_offline: true,
      local_pre_authorize: false,
      authorization_cache_enabled: true,
      allow_offline_tx_for_unknown_id: false,

      // 💳 Local Auth List
      local_auth_list_enabled: false,
      local_auth_list_version: 0,
      send_local_list_max_length: 100,
      local_auth_list_max_length: 100,

      // 🔌 Connector / Hardware
      number_of_connectors: 1,
      connector_phase_rotation: "Unknown".to_string(),

      // 🌐 Network / WebSocket
      websocket_ping_interval: 0,

      // ⚡ Smart Charging
      charge_profile_max_stack_level: 10,
      charging_schedule_allowed_charging_rate_unit: "Current,Power".to_string(),
      charging_schedule_max_periods: 24,

      // 📊 Limits / Misc
      get_configuration_max_keys: 50,
    }
  }
}

pub struct ChargingSessionState {
  pub energy_wh: f64,
  pub last_update: Instant,
}

impl Default for ChargingSessionState {
  fn default() -> Self {
    Self {
      energy_wh: 0.0,
      last_update: Instant::now(),
    }
  }
}

pub struct SharedState<A: SharedDataValue> {
  msgs: HashMap<MsgId, A>,

  pub transaction_id: Option<i32>,
  pub tag_id: Option<TagId>,

  pub charging_session_state: ChargingSessionState,
  pub settings: ChargePointSettings,
}

impl<A: SharedDataValue> SharedState<A> {
  fn new() -> Self {
    Self {
      msgs: HashMap::new(),
      transaction_id: None,
      tag_id: None,
      charging_session_state: ChargingSessionState::default(),
      settings: ChargePointSettings::default(),
    }
  }
}

#[derive(Clone)]
pub struct SharedData<A: SharedDataValue> {
  state: Arc<RwLock<SharedState<A>>>,
}

impl<A: SharedDataValue + Clone> Default for SharedData<A> {
  fn default() -> Self {
    Self {
      state: Arc::new(RwLock::new(SharedState::new())),
    }
  }
}

impl<A: SharedDataValue + Clone> SharedData<A> {
  pub async fn from_cp_config(value: &ChargePointConfig) -> Self {
    let state = Arc::new(RwLock::new(SharedState::new()));
    state.write().await.tag_id = Some(value.id_tag.clone());
    Self { state }
  }

  pub async fn get_msg(&self, msg_id: &str) -> Option<A> {
    self.state.read().await.msgs.get(msg_id).cloned()
  }

  pub async fn insert_msg(&self, msg_id: &String, ocpp_action: A) {
    self
      .state
      .write()
      .await
      .msgs
      .insert(msg_id.to_string().clone(), ocpp_action);
  }

  pub async fn read<F, R>(&self, f: F) -> R
  where
    F: FnOnce(&SharedState<A>) -> R,
  {
    f(&*self.state.read().await)
  }

  pub async fn write<F, R>(&self, f: F) -> R
  where
    F: FnOnce(&mut SharedState<A>) -> R,
  {
    f(&mut *self.state.write().await)
  }
}
