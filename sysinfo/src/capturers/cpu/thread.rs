use std::ops::{AddAssign, DivAssign, SubAssign};

#[derive(Clone)]
pub struct Thread {
  // Its normalized to 1.0-0.0 range,
  // 1.0 mean online, 0.0 mean offline
  //
  // If this sample from average
  // it also adds meaning, 0.8 mean the
  // thread online 80% on average
  pub online_percent: f64,
  pub utilization: f64,
  pub frequency_khz: f64
}

impl<'a> AddAssign<&'a Self> for Thread {
  fn add_assign(&mut self, rhs: &'a Self) {
    self.utilization += rhs.utilization;
    self.frequency_khz += rhs.frequency_khz;
  }
}

impl<'a> SubAssign<&'a Self> for Thread {
  fn sub_assign(&mut self, rhs: &'a Self) {
    self.utilization -= rhs.utilization;
    self.frequency_khz -= rhs.frequency_khz;
  }
}

impl DivAssign<f64> for Thread {
  fn div_assign(&mut self, rhs: f64) {
    self.utilization /= rhs;
    self.frequency_khz /= rhs;
  }
}



