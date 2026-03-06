mod metric;
mod capturers;
mod util;

use metric::Metric;

use crate::capturers::{cpu::CpuCapture, swap::SwapCapture};

pub struct SysInfo {
  pub cpu_usage: Metric<CpuCapture>,
  pub swap_usage: Metric<SwapCapture>
}

impl SysInfo {
  pub fn new() -> Self {
    Self {
      cpu_usage: Metric::new(CpuCapture::new(), 30),
      swap_usage: Metric::new(SwapCapture::new(), 30)
    }
  }
  
  pub fn update(&mut self) {
    self.cpu_usage.update();
    self.swap_usage.update();
  }
}

