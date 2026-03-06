use std::ops::{AddAssign, DivAssign, SubAssign};

#[derive(Clone)]
pub struct Thread {
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



