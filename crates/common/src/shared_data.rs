use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

pub trait SharedDataValue: Send + Sync {}
impl <A: Send + Sync> SharedDataValue for A {}

struct SharedState<A: SharedDataValue> {
  pending_requests: HashMap<String, A>,
  transaction_id: Option<i32>
}

impl<A: SharedDataValue> SharedState<A> {
  fn new() -> Self {
    Self {
      pending_requests: HashMap::new(),
      transaction_id: None
    }
  }
}

#[derive(Clone)]
pub struct SharedData<A: SharedDataValue> {
  state: Arc<RwLock<SharedState<A>>>
}

impl<A: SharedDataValue> SharedData<A> {
  pub fn new() -> Self {
    Self {
      state: Arc::new(RwLock::new(SharedState::new()))
    }
  }

  pub async fn get_transaction_id(&self) -> Option<i32> {
    let state  = self.state.read().await;
    state.transaction_id
  }

  pub async fn transaction_id(&self, transaction_id: i32) {
    self.state.write().await.transaction_id = Some(transaction_id);
  }
}

