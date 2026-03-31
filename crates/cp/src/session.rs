use std::future::Future;
use std::time::Duration;

use tokio::time::{Instant, Interval, MissedTickBehavior, interval};

pub struct TxnSession {
  is_running: bool,
  max_count: u64,
  count: u64,
  interval: Interval,
}

impl TxnSession {
  pub fn new(meter_values_interval: u64, meter_values_max_cnt: u64) -> Self {
    let mut interval = interval(Duration::from_secs(meter_values_interval));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    Self {
      is_running: false,
      max_count: meter_values_max_cnt,
      count: 0,
      interval,
    }
  }

  pub fn is_running(&self) -> bool {
    self.is_running
  }

  pub fn tick(&mut self) -> impl Future<Output = Instant> {
    self.interval.tick()
  }

  /// Simulates `meter_values` internal counter
  pub fn increment(&mut self) {
    self.count += 1;

    if self.count >= self.max_count {
      self.stop();
    }
  }

  pub fn start(&mut self) {
    self.count = 0;
    self.is_running = true;
    self.interval.reset();
  }

  pub fn stop(&mut self) {
    self.is_running = false;
    self.count = 0;
  }
}
