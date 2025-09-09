use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

pub trait SharedDataValue: Send + Sync {}
impl<A: Send + Sync> SharedDataValue for A {}

struct SharedState<A: SharedDataValue> {
  msgs: HashMap<String, A>,
  transaction_id: Option<i32>,
}

impl<A: SharedDataValue> SharedState<A> {
  fn new() -> Self {
    Self {
      msgs: HashMap::new(),
      transaction_id: None,
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
  pub async fn insert_msg(&self, msg_id: &String, ocpp_action: A) {
    self
      .state
      .write()
      .await
      .msgs
      .insert(msg_id.to_string().clone(), ocpp_action);
  }

  pub async fn get_msg(&self, msg_id: &str) -> Option<A> {
    self.state.read().await.msgs.get(msg_id).cloned()
  }

  pub async fn get_transaction_id(&self) -> Option<i32> {
    let state = self.state.read().await;
    state.transaction_id
  }

  pub async fn transaction_id(&self, transaction_id: i32) {
    self.state.write().await.transaction_id = Some(transaction_id);
  }
}
