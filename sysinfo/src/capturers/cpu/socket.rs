use std::ops::{AddAssign, DivAssign, SubAssign};

use crate::capturers::cpu::Cluster;

#[derive(Clone)]
pub struct Socket {
  pub utilization: f64,
  pub frequency_khz: f64,
  pub clusters: Vec<Cluster>
}

impl Socket {
  pub(crate) fn sanify(&mut self) {
    self.clusters.iter_mut().for_each(Cluster::sanify);
    self.utilization = self.clusters.iter()
      .fold(0.0, |acc, x| {
        acc + x.utilization
      }) / self.clusters.len() as f64;
    
    self.frequency_khz = self.clusters.iter()
      .fold(0.0, |acc, x| {
        acc + x.frequency_khz
      }) / self.clusters.len() as f64;
  }
}

impl DivAssign<f64> for Socket {
  fn div_assign(&mut self, rhs: f64) {
    self.utilization /= rhs;
    self.frequency_khz /= rhs;
    self.clusters.iter_mut()
      .for_each(|node| {
        *node /= rhs;
      });
  }
}

impl<'a> AddAssign<&'a Self> for Socket {
  fn add_assign(&mut self, rhs: &'a Self) {
    self.frequency_khz += rhs.frequency_khz;
    self.utilization += rhs.utilization;
    self.clusters.iter_mut()
      .zip(rhs.clusters.iter())
      .for_each(|(node, rhs)| {
        *node += rhs;
      });
  }
}

impl<'a> SubAssign<&'a Self> for Socket {
  fn sub_assign(&mut self, rhs: &'a Self) {
    self.frequency_khz -= rhs.frequency_khz;
    self.utilization -= rhs.utilization;
    self.clusters.iter_mut()
      .zip(rhs.clusters.iter())
      .for_each(|(node, rhs)| {
        *node -= rhs;
      });
  }
}


