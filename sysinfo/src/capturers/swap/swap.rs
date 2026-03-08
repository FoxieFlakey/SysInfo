use std::ops::{AddAssign, DivAssign, SubAssign};

use crate::c_api::cstring::CString;

#[derive(Clone)]
pub struct SwapDev {
  pub path: CString,
  pub size_kib: f64,
  pub used_kib: f64
}

impl SwapDev {
  pub(crate) fn do_min_on_all_fields(&mut self, rhs: &Self) {
    self.size_kib = f64::min(self.size_kib, rhs.size_kib);
    self.used_kib = f64::min(self.used_kib, rhs.used_kib);
  }
  
  pub(crate) fn do_max_on_all_fields(&mut self, rhs: &Self) {
    self.size_kib = f64::max(self.size_kib, rhs.size_kib);
    self.used_kib = f64::max(self.used_kib, rhs.used_kib);
  }
}

impl<'a> AddAssign<&'a Self> for SwapDev {
  fn add_assign(&mut self, rhs: &'a Self) {
    self.size_kib += rhs.size_kib;
    self.used_kib += rhs.used_kib;
  }
}

impl<'a> SubAssign<&'a Self> for SwapDev {
  fn sub_assign(&mut self, rhs: &'a Self) {
    self.size_kib -= rhs.size_kib;
    self.used_kib -= rhs.used_kib;
  }
}

impl<'a> DivAssign<f64> for SwapDev {
  fn div_assign(&mut self, rhs: f64) {
    self.size_kib /= rhs;
    self.used_kib /= rhs;
  }
}

