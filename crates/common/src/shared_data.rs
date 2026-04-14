use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::ChargePointConfig;

type MsgId = String;
type TagId = String;

pub trait SharedDataValue: Send + Sync {}
impl<A: Send + Sync> SharedDataValue for A {}

pub struct SharedState<A: SharedDataValue> {
  msgs: HashMap<MsgId, A>,
  pub transaction_id: Option<i32>,
  pub tag_id: Option<TagId>,
  pub meter_stop: u32,
}

impl<A: SharedDataValue> SharedState<A> {
  fn new() -> Self {
    Self {
      msgs: HashMap::new(),
      transaction_id: None,
      tag_id: None,
      meter_stop: 0,
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
