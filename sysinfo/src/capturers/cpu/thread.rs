use std::ops::{AddAssign, DivAssign, SubAssign};

#[repr(C)]
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

impl Thread {
  pub(crate) fn do_max_on_all_fields(&mut self, rhs: &Self) {
    self.frequency_khz = f64::max(self.frequency_khz, rhs.frequency_khz);
    self.utilization = f64::max(self.utilization, rhs.utilization);
  }
  
  pub(crate) fn do_min_on_all_fields(&mut self, rhs: &Self) {
    self.frequency_khz = f64::min(self.frequency_khz, rhs.frequency_khz);
    self.utilization = f64::min(self.utilization, rhs.utilization);
  }
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



