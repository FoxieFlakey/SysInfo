use std::ops::{AddAssign, DivAssign, SubAssign};

use crate::metric::Sample;

mod socket;
mod cluster;
mod core;
mod thread;
mod capture;
mod die;
pub use {socket::Socket, cluster::Cluster, core::Core, thread::Thread, capture::CpuCapture, die::Die};

#[derive(Clone)]
pub struct CPU {
  pub utilization: f64,
  pub frequency_khz: f64,
  pub sockets: Vec<Socket>
}

impl CPU {
  pub(crate) fn sanify(&mut self) {
    self.sockets.iter_mut().for_each(Socket::sanify);
    
    self.utilization = self.sockets.iter()
      .fold(0.0, |acc, x| {
        acc + x.utilization
      }) / self.sockets.len() as f64;
    
    self.frequency_khz = self.sockets.iter()
      .fold(0.0, |acc, x| {
        acc + x.frequency_khz
      }) / self.sockets.len() as f64;
  }
}

impl<'a> SubAssign<&'a Self> for CPU {
  fn sub_assign(&mut self, rhs: &'a Self) {
    self.frequency_khz -= rhs.frequency_khz;
    self.utilization -= rhs.utilization;
    self.sockets.iter_mut()
      .zip(rhs.sockets.iter())
      .for_each(|(lhs, rhs)| {
        *lhs += rhs;
      });
  }
}

impl<'a> AddAssign<&'a Self> for CPU {
  fn add_assign(&mut self, rhs: &'a Self) {
    self.frequency_khz += rhs.frequency_khz;
    self.utilization += rhs.utilization;
    self.sockets.iter_mut()
      .zip(rhs.sockets.iter())
      .for_each(|(lhs, rhs)| {
        *lhs += rhs;
      });
  }
}

impl<'a> DivAssign<f64> for CPU {
  fn div_assign(&mut self, rhs: f64) {
    self.frequency_khz /= rhs;
    self.utilization /= rhs;
    self.sockets.iter_mut()
      .for_each(|socket| {
        *socket /= rhs;
      });
  }
}

impl Sample<'_> for CPU {
  fn do_max_on_all_fields(&mut self, rhs: &Self) {
    self.frequency_khz = f64::max(self.frequency_khz, rhs.frequency_khz);
    self.utilization = f64::max(self.utilization, rhs.utilization);
  }
  
  fn do_min_on_all_fields(&mut self, rhs: &Self) {
    self.frequency_khz = f64::min(self.frequency_khz, rhs.frequency_khz);
    self.utilization = f64::min(self.utilization, rhs.utilization);
  }
}

