use std::ops::{AddAssign, DivAssign, SubAssign};

use crate::capturers::cpu::Cluster;

#[derive(Clone)]
pub struct Die {
  pub utilization: f64,
  pub frequency_khz: f64,
  pub clusters: Vec<Cluster>
}

impl Die {
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

impl DivAssign<f64> for Die {
  fn div_assign(&mut self, rhs: f64) {
    self.utilization /= rhs;
    self.frequency_khz /= rhs;
    self.clusters.iter_mut()
      .for_each(|cluster| {
        *cluster /= rhs;
      });
  }
}

impl<'a> AddAssign<&'a Self> for Die {
  fn add_assign(&mut self, rhs: &'a Self) {
    self.frequency_khz += rhs.frequency_khz;
    self.utilization += rhs.utilization;
    self.clusters.iter_mut()
      .zip(rhs.clusters.iter())
      .for_each(|(cluster, rhs)| {
        *cluster += rhs;
      });
  }
}

impl<'a> SubAssign<&'a Self> for Die {
  fn sub_assign(&mut self, rhs: &'a Self) {
    self.frequency_khz -= rhs.frequency_khz;
    self.utilization -= rhs.utilization;
    self.clusters.iter_mut()
      .zip(rhs.clusters.iter())
      .for_each(|(cluster, rhs)| {
        *cluster -= rhs;
      });
  }
}

