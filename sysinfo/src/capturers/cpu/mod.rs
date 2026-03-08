use std::ops::{AddAssign, DivAssign, SubAssign};

use crate::{c_api::cvec::CVec, metric::Sample};

mod socket;
mod cluster;
mod core;
mod thread;
mod capture;
mod die;
mod stat;

pub use {socket::Socket, cluster::Cluster, core::Core, thread::Thread, capture::CpuCapture, die::Die};

#[repr(C)]
#[derive(Clone)]
pub struct CPU {
  pub utilization: f64,
  pub frequency_khz: f64,
  
  // See Linux doc at https://www.kernel.org/doc/Documentation/ABI/testing/sysfs-devices-system-cpu
  // Equivalent to /sys/devices/system/cpu/present
  pub present: f64,
  // Equivalent to /sys/devices/system/cpu/possible
  pub possible: f64,
  // Equivalent to /sys/devices/system/cpu/online
  pub online: f64,
  // Equivalent to /sys/devices/system/cpu/offline
  pub offline: f64,
  
  pub sockets: CVec<Socket>
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
    
    self.present -= rhs.present;
    self.possible -= rhs.possible;
    self.online -= rhs.online;
    self.offline -= rhs.offline;
    
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
    
    self.present += rhs.present;
    self.possible += rhs.possible;
    self.online += rhs.online;
    self.offline += rhs.offline;
    
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
    
    self.present /= rhs;
    self.possible /= rhs;
    self.online /= rhs;
    self.offline /= rhs;
    
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
    
    self.sockets.iter_mut()
      .zip(rhs.sockets.iter())
      .for_each(|(rhs, lhs)| rhs.do_max_on_all_fields(lhs));
  }
  
  fn do_min_on_all_fields(&mut self, rhs: &Self) {
    self.frequency_khz = f64::min(self.frequency_khz, rhs.frequency_khz);
    self.utilization = f64::min(self.utilization, rhs.utilization);
    
    self.sockets.iter_mut()
      .zip(rhs.sockets.iter())
      .for_each(|(rhs, lhs)| rhs.do_min_on_all_fields(lhs));
  }
}

