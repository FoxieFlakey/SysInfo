use std::path::Path;

use crate::{capturers::loadavg::LoadAvg, metric::Capturer, util};

pub struct LoadAvgCapturer;

impl LoadAvgCapturer {
  pub fn new() -> Self {
    Self
  }
}

impl Capturer for LoadAvgCapturer {
  type Sample = LoadAvg;
  
  fn capture(&mut self) -> Option<Self::Sample> {
    let string = util::read_all(Path::new("/proc/loadavg")).ok()?;
    let mut fields = string.split_whitespace();
    let load_1m = fields.next()?;
    let load_5m = fields.next()?;
    let load_15m = fields.next()?;
    
    let (runnable_task, total_task) = {
      let tasks = fields.next()?;
      let mut tasks_fields = tasks.split('/');
      
      (tasks_fields.next()?, tasks_fields.next()?)
    };
    
    if
      load_1m.starts_with('-') ||
      load_5m.starts_with('-') ||
      load_15m.starts_with('-') ||
      runnable_task.starts_with('-') ||
      total_task.starts_with('-')
    {
      return None;
    }
    
    Some(LoadAvg {
      load_1m: load_1m.parse::<f64>().ok()?,
      load_5m: load_5m.parse::<f64>().ok()?,
      load_15m: load_15m.parse::<f64>().ok()?,
      runnable_task_count: runnable_task.parse::<f64>().ok()?,
      task_count: total_task.parse::<f64>().ok()?
    })
  }
}


