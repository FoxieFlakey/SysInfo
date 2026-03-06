use std::ops::{AddAssign, DivAssign, SubAssign};

use crate::metric::Sample;

mod swap;
mod capture;

pub use {swap::SwapDev, capture::SwapCapture};

#[derive(Clone)]
pub struct Swaps {
  pub total_size_kib: f64,
  pub total_used_kib: f64,
  pub swaps: Vec<SwapDev>
}

impl Swaps {
  pub(crate) fn sanify(&mut self) {
    let mut total_size = 0.0;
    let mut total_used = 0.0;
    
    self.swaps.iter()
      .for_each(|x| {
        total_size += x.size_kib;
        total_used += x.used_kib;
      });
    
    self.total_size_kib = total_size;
    self.total_used_kib = total_used;
  }
}

impl<'a> AddAssign<&'a Self> for Swaps {
  fn add_assign(&mut self, rhs: &'a Self) {
    self.total_size_kib += rhs.total_size_kib;
    self.total_used_kib += rhs.total_used_kib;
    
    self.swaps.iter_mut()
      .zip(rhs.swaps.iter())
      .for_each(|(lhs, rhs)| {
        *lhs += rhs;
      });
  }
}

impl<'a> SubAssign<&'a Self> for Swaps {
  fn sub_assign(&mut self, rhs: &'a Self) {
    self.total_size_kib -= rhs.total_size_kib;
    self.total_used_kib -= rhs.total_used_kib;
    
    self.swaps.iter_mut()
      .zip(rhs.swaps.iter())
      .for_each(|(lhs, rhs)| {
        *lhs -= rhs;
      });
  }
}

impl<'a> DivAssign<f64> for Swaps {
  fn div_assign(&mut self, rhs: f64) {
    self.total_size_kib /= rhs;
    self.total_used_kib /= rhs;
    
    self.swaps.iter_mut()
      .for_each(|lhs| {
        *lhs /= rhs;
      });
  }
}

impl<'a> Sample<'a> for Swaps {
  fn do_min_on_all_fields(&mut self, rhs: &Self) {
    self.total_size_kib = f64::min(self.total_size_kib, rhs.total_size_kib);
    self.total_used_kib = f64::min(self.total_used_kib, rhs.total_used_kib);
    
    self.swaps.iter_mut()
      .zip(rhs.swaps.iter())
      .for_each(|(lhs, rhs)| SwapDev::do_min_on_all_fields(lhs, rhs));
  }

  fn do_max_on_all_fields(&mut self, rhs: &Self) {
    self.total_size_kib = f64::max(self.total_size_kib, rhs.total_size_kib);
    self.total_used_kib = f64::max(self.total_used_kib, rhs.total_used_kib);
    
    self.swaps.iter_mut()
      .zip(rhs.swaps.iter())
      .for_each(|(lhs, rhs)| SwapDev::do_max_on_all_fields(lhs, rhs));
  }
}


