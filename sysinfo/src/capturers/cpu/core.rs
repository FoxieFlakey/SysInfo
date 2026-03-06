use std::ops::{AddAssign, DivAssign, SubAssign};

use crate::capturers::cpu::Thread;

#[derive(Clone)]
pub struct Core {
  pub utilization: f64,
  pub frequency_khz: f64,
  pub threads: Vec<Thread>
}

impl Core {
  pub(crate) fn do_max_on_all_fields(&mut self, rhs: &Self) {
    self.frequency_khz = f64::max(self.frequency_khz, rhs.frequency_khz);
    self.utilization = f64::max(self.utilization, rhs.utilization);
    
    self.threads.iter_mut()
      .zip(rhs.threads.iter())
      .for_each(|(rhs, lhs)| rhs.do_max_on_all_fields(lhs));
  }
  
  pub(crate) fn do_min_on_all_fields(&mut self, rhs: &Self) {
    self.frequency_khz = f64::min(self.frequency_khz, rhs.frequency_khz);
    self.utilization = f64::min(self.utilization, rhs.utilization);
    
    self.threads.iter_mut()
      .zip(rhs.threads.iter())
      .for_each(|(rhs, lhs)| rhs.do_min_on_all_fields(lhs));
  }
  
  pub(crate) fn sanify(&mut self) {
    self.utilization = self.threads.iter()
      .fold(0.0, |acc, x| {
        acc + x.utilization
      }) / self.threads.len() as f64;
    
    self.frequency_khz = self.threads.iter()
      .fold(0.0, |acc, x| {
        acc + x.frequency_khz
      }) / self.threads.len() as f64;
  }
}

impl<'a> AddAssign<&'a Self> for Core {
  fn add_assign(&mut self, rhs: &'a Self) {
    self.utilization += rhs.utilization;
    self.frequency_khz += rhs.frequency_khz;
    self.threads
      .iter_mut()
      .zip(rhs.threads.iter())
      .for_each(|(thread, rhs)| {
        *thread += rhs;
      });
  }
}

impl<'a> SubAssign<&'a Self> for Core {
  fn sub_assign(&mut self, rhs: &'a Self) {
    self.utilization -= rhs.utilization;
    self.frequency_khz -= rhs.frequency_khz;
    self.threads
      .iter_mut()
      .zip(rhs.threads.iter())
      .for_each(|(thread, rhs)| {
        *thread -= rhs;
      });
  }
}

impl DivAssign<f64> for Core {
  fn div_assign(&mut self, rhs: f64) {
    self.utilization /= rhs;
    self.frequency_khz /= rhs;
    self.threads
      .iter_mut()
      .for_each(|thread| {
        *thread /= rhs;
      });
  }
}

