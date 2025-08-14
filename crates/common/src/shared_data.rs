use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

struct SharedState<A: Send + Sync> {
  pending_requests: HashMap<String, A>,
  transaction_id: Option<i32>
}

impl<A: Send + Sync> SharedState<A> {
  fn new() -> Self {
    Self {
      pending_requests: HashMap::new(),
      transaction_id: None
    }
  }
}

#[derive(Clone)]
pub struct SharedData<A: Send + Sync> {
  state: Arc<RwLock<SharedState<A>>>
}

impl<A: Send + Sync> SharedData<A> {
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

