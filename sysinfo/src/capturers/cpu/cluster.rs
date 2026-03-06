use std::ops::{AddAssign, DivAssign, SubAssign};

use crate::capturers::cpu::Core;

#[derive(Clone)]
pub struct Cluster {
  pub utilization: f64,
  pub frequency_khz: f64,
  pub cores: Vec<Core>
}

impl Cluster {
  pub(crate) fn do_max_on_all_fields(&mut self, rhs: &Self) {
    self.frequency_khz = f64::max(self.frequency_khz, rhs.frequency_khz);
    self.utilization = f64::max(self.utilization, rhs.utilization);
    
    self.cores.iter_mut()
      .zip(rhs.cores.iter())
      .for_each(|(rhs, lhs)| rhs.do_max_on_all_fields(lhs));
  }
  
  pub(crate) fn do_min_on_all_fields(&mut self, rhs: &Self) {
    self.frequency_khz = f64::min(self.frequency_khz, rhs.frequency_khz);
    self.utilization = f64::min(self.utilization, rhs.utilization);
    
    self.cores.iter_mut()
      .zip(rhs.cores.iter())
      .for_each(|(rhs, lhs)| rhs.do_min_on_all_fields(lhs));
  }
  
  pub fn sanify(&mut self) {
    self.cores.iter_mut().for_each(Core::sanify);
    self.utilization = self.cores.iter()
      .fold(0.0, |acc, x| {
        acc + x.utilization
      }) / self.cores.len() as f64;
    
    self.frequency_khz = self.cores.iter()
      .fold(0.0, |acc, x| {
        acc + x.frequency_khz
      }) / self.cores.len() as f64;
  }
}

impl<'a> AddAssign<&'a Self> for Cluster {
  fn add_assign(&mut self, rhs: &'a Self) {
    self.utilization += rhs.utilization;
    self.frequency_khz += rhs.frequency_khz;
    self.cores.iter_mut()
      .zip(rhs.cores.iter())
      .for_each(|(core, rhs)| {
        *core += rhs;
      });
  }
}

impl<'a> SubAssign<&'a Self> for Cluster {
  fn sub_assign(&mut self, rhs: &'a Self) {
    self.utilization -= rhs.utilization;
    self.frequency_khz -= rhs.frequency_khz;
    self.cores.iter_mut()
      .zip(rhs.cores.iter())
      .for_each(|(core, rhs)| {
        *core -= rhs;
      });
  }
}

impl DivAssign<f64> for Cluster {
  fn div_assign(&mut self, rhs: f64) {
    self.utilization /= rhs;
    self.frequency_khz /= rhs;
    self.cores.iter_mut()
      .for_each(|core| {
        *core /= rhs;
      });
  }
}

