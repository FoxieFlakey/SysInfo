mod capturer;
use std::ops::{AddAssign, DivAssign, SubAssign};

pub use capturer::LoadAvgCapturer;

use crate::metric::Sample;

#[derive(Clone)]
pub struct LoadAvg {
  pub load_1m: f64,
  pub load_5m: f64,
  pub load_15m: f64,
  pub runnable_task_count: f64,
  pub task_count: f64
}

impl<'a> SubAssign<&'a Self> for LoadAvg {
  fn sub_assign(&mut self, rhs: &'a Self) {
    self.load_1m -= rhs.load_1m;
    self.load_5m -= rhs.load_5m;
    self.load_15m -= rhs.load_15m;
    self.runnable_task_count -= rhs.runnable_task_count;
    self.task_count -= rhs.task_count;
  }
}

impl<'a> AddAssign<&'a Self> for LoadAvg {
  fn add_assign(&mut self, rhs: &'a Self) {
    self.load_1m += rhs.load_1m;
    self.load_5m += rhs.load_5m;
    self.load_15m += rhs.load_15m;
    self.runnable_task_count += rhs.runnable_task_count;
    self.task_count += rhs.task_count;
  }
}

impl DivAssign<f64> for LoadAvg {
  fn div_assign(&mut self, rhs: f64) {
    self.load_1m /= rhs;
    self.load_5m /= rhs;
    self.load_15m /= rhs;
    self.runnable_task_count /= rhs;
    self.task_count /= rhs;
  }
}

impl Sample<'_> for LoadAvg {
  fn do_max_on_all_fields(&mut self, rhs: &Self) {
    self.load_1m = f64::max(self.load_1m, rhs.load_1m);
    self.load_5m = f64::max(self.load_5m, rhs.load_5m);
    self.load_15m = f64::max(self.load_15m, rhs.load_15m);
    self.runnable_task_count = f64::max(self.runnable_task_count, rhs.runnable_task_count);
    self.task_count = f64::max(self.task_count, rhs.task_count);
  }
  
  fn do_min_on_all_fields(&mut self, rhs: &Self) {
    self.load_1m = f64::min(self.load_1m, rhs.load_1m);
    self.load_5m = f64::min(self.load_5m, rhs.load_5m);
    self.load_15m = f64::min(self.load_15m, rhs.load_15m);
    self.runnable_task_count = f64::min(self.runnable_task_count, rhs.runnable_task_count);
    self.task_count = f64::min(self.task_count, rhs.task_count);
  }
}


