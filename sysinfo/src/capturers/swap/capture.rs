use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use crate::{c_api::cvec::CVec, capturers::swap::{SwapDev, Swaps}, metric::Capturer};

pub struct SwapCapture;

impl SwapCapture {
  pub fn new() -> Self {
    Self
  }
}

impl Capturer for SwapCapture {
  type Sample = Swaps;
  
  fn capture(&mut self) -> Option<Self::Sample> {
    let file = File::open(Path::new("/proc/swaps")).ok()?;
    let mut lines = BufReader::new(file).lines();
    
    {
      let line = lines.next()?.ok()?;
      let mut header = line.split_ascii_whitespace();
      
      if 
        header.next()? != "Filename" ||
        header.next()? != "Type" ||
        header.next()? != "Size" ||
        header.next()? != "Used"
      {
        return None;
      }
    }
    
    let mut devs = CVec::new();
    for line in lines {
      let line = line.ok()?;
      let mut fields = line.split_ascii_whitespace();
      let filename = fields.next()?;
      let _type = fields.next()?;
      let size = fields.next()?;
      let used = fields.next()?;
      
      if size.starts_with("-") || used.starts_with("-") {
        return None;
      }
      
      let size = u32::from_str_radix(size, 10).ok()?;
      let used = u32::from_str_radix(used, 10).ok()?;
      
      devs.push(SwapDev {
        path: filename.to_string(),
        size_kib: f64::from(size),
        used_kib: f64::from(used)
      });
    }
    
    let mut swaps = Swaps {
      total_size_kib: 0.0,
      total_used_kib: 0.0,
      swaps: devs
    };
    swaps.sanify();
    Some(swaps)
  }
}


