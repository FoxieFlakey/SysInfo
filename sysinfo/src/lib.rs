mod metric;
mod capturers;
mod util;

use metric::Metric;

use crate::capturers::cpu::CpuCapture;

pub struct SysInfo {
  pub cpu_usage: Metric<CpuCapture>
}

impl SysInfo {
  pub fn new() -> Self {
    Self {
      cpu_usage: Metric::new(CpuCapture::new(), 30)
    }
  }
  
  pub fn update(&mut self) {
    self.cpu_usage.update();
  }
}

