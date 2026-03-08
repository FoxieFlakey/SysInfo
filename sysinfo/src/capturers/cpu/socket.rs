use std::ops::{AddAssign, DivAssign, SubAssign};

use crate::{c_api::cvec::CVec, capturers::cpu::Die};

#[repr(C)]
#[derive(Clone)]
pub struct Socket {
  pub utilization: f64,
  pub frequency_khz: f64,
  pub dies: CVec<Die>
}

impl Socket {
  pub(crate) fn do_max_on_all_fields(&mut self, rhs: &Self) {
    self.frequency_khz = f64::max(self.frequency_khz, rhs.frequency_khz);
    self.utilization = f64::max(self.utilization, rhs.utilization);
    
    self.dies.iter_mut()
      .zip(rhs.dies.iter())
      .for_each(|(rhs, lhs)| rhs.do_max_on_all_fields(lhs));
  }
  
  pub(crate) fn do_min_on_all_fields(&mut self, rhs: &Self) {
    self.frequency_khz = f64::min(self.frequency_khz, rhs.frequency_khz);
    self.utilization = f64::min(self.utilization, rhs.utilization);
    
    self.dies.iter_mut()
      .zip(rhs.dies.iter())
      .for_each(|(rhs, lhs)| rhs.do_min_on_all_fields(lhs));
  }
  
  pub(crate) fn sanify(&mut self) {
    self.dies.iter_mut().for_each(Die::sanify);
    self.utilization = self.dies.iter()
      .fold(0.0, |acc, x| {
        acc + x.utilization
      }) / self.dies.len() as f64;
    
    self.frequency_khz = self.dies.iter()
      .fold(0.0, |acc, x| {
        acc + x.frequency_khz
      }) / self.dies.len() as f64;
  }
}

impl DivAssign<f64> for Socket {
  fn div_assign(&mut self, rhs: f64) {
    self.utilization /= rhs;
    self.frequency_khz /= rhs;
    self.dies.iter_mut()
      .for_each(|die| {
        *die /= rhs;
      });
  }
}

impl<'a> AddAssign<&'a Self> for Socket {
  fn add_assign(&mut self, rhs: &'a Self) {
    self.frequency_khz += rhs.frequency_khz;
    self.utilization += rhs.utilization;
    self.dies.iter_mut()
      .zip(rhs.dies.iter())
      .for_each(|(die, rhs)| {
        *die += rhs;
      });
  }
}

impl<'a> SubAssign<&'a Self> for Socket {
  fn sub_assign(&mut self, rhs: &'a Self) {
    self.frequency_khz -= rhs.frequency_khz;
    self.utilization -= rhs.utilization;
    self.dies.iter_mut()
      .zip(rhs.dies.iter())
      .for_each(|(die, rhs)| {
        *die -= rhs;
      });
  }
}


