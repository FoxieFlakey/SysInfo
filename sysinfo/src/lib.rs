mod metric;
mod capturers;
mod util;

use std::{thread, time::Duration};

use metric::Metric;

use crate::capturers::{cpu::CpuCapture, memory::MemoryCapturer, swap::SwapCapture, loadavg::LoadAvgCapturer};

pub struct SysInfo {
  pub cpu_usage: Metric<CpuCapture>,
  pub swap_usage: Metric<SwapCapture>,
  pub memory_usage: Metric<MemoryCapturer>,
  pub loadavg: Metric<LoadAvgCapturer>
}

impl SysInfo {
  pub fn new() -> Self {
    Self {
      cpu_usage: Metric::new(CpuCapture::new(), 30),
      swap_usage: Metric::new(SwapCapture::new(), 30),
      memory_usage: Metric::new(MemoryCapturer::new(), 30),
      
      // Note: /proc/loadavg is already averaged and doesn't really need historical samples
      loadavg: Metric::new(LoadAvgCapturer::new(), 1)
    }
  }
  
  pub fn update(&mut self) {
    self.cpu_usage.prep_update();
    self.swap_usage.prep_update();
    self.memory_usage.prep_update();
    self.loadavg.prep_update();
    
    thread::sleep(Duration::from_millis(700));
    
    self.cpu_usage.update();
    self.swap_usage.update();
    self.memory_usage.update();
    self.loadavg.update();
  }
}

