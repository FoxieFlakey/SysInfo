use std::{fs::File, io::{BufRead, BufReader}};

// Parses /proc/stat
#[derive(Clone)]
pub struct ProcStat {
  pub total_time: CpuTime,
  pub cpus: Vec<CpuTime>
}

fn parse_positive_u64(str: &str) -> Option<u64> {
  if str.starts_with('-') {
    // Invalid ID was found
    return None;
  }
  
  u64::from_str_radix(str, 10).ok()
}

impl ProcStat {
  pub fn capture() -> Option<Self> {
    let file = File::open("/proc/stat").ok()?;
    let mut lines = BufReader::new(file).lines();
    
    let total_time = CpuTime::parse(lines.next()?.ok()?)?;
    let mut cpus = Vec::new();
    let mut prev_id = None;
    
    for line in lines {
      let line = line.ok()?;
      if !line.starts_with("cpu") {
        continue;
      }
      
      // Offset it so first entry after split supposed to be integer part
      // of cpuN
      let id = line[3..].split_whitespace().next()?;
      if id.starts_with('-') {
        // Invalid ID was found
        return None;
      }
      
      let id = u32::from_str_radix(id, 10).ok()?;
      if let Some(prev) = prev_id {
        if id <= prev {
          // Not ordered properly... 
          return None;
        }
      } else {
        if id != 0 {
          // Not ordered properly...
          return None;
        }
        
        prev_id = Some(id);
      }
      
      cpus.push(CpuTime::parse(line)?);
    }
    
    Some(ProcStat { total_time, cpus })
  }
}

// Represent single line of data for
// cpu .. .. ... or cpuX .. ... ..
//
// The unit is not really defined, only
// their relation between other is defined
//
// See https://man7.org/linux/man-pages/man5/proc_stat.5.html
#[derive(Clone, Default)]
pub struct CpuTime {
  pub user_time: u64,
  pub nice_time: u64,
  pub sys_time: u64,
  pub idle_time: u64,
  pub iowait_time: u64,
  pub irq_time: u64,
  pub softirq_time: u64,
  pub steal_time: u64,
  pub guest_time: u64,
  pub guest_nice_time: u64
}

impl CpuTime {
  // This ignores the first field
  fn parse(line: String) -> Option<Self> {
    let mut fields = line.split_whitespace();
    fields.next()?;
    
    Some(Self {
      user_time: parse_positive_u64(fields.next()?)?,
      nice_time: parse_positive_u64(fields.next()?)?,
      sys_time: parse_positive_u64(fields.next()?)?,
      idle_time: parse_positive_u64(fields.next()?)?,
      iowait_time: parse_positive_u64(fields.next()?)?,
      irq_time: parse_positive_u64(fields.next()?)?,
      softirq_time: parse_positive_u64(fields.next()?)?,
      steal_time: parse_positive_u64(fields.next()?)?,
      guest_time: parse_positive_u64(fields.next()?)?,
      guest_nice_time: parse_positive_u64(fields.next()?)?,
    })
  }
  
  // iowait is excluded intentionally due CPU
  // will likely execute other thing, see
  // https://man7.org/linux/man-pages/man5/proc_stat.5.html
  // for iowait field
  //
  // Excludes idle because this suppose to only include active
  // states which mean executing instructions
  pub fn combine_all_active(&self) -> u64 {
    self.user_time +
    self.nice_time +
    self.sys_time +
    self.irq_time +
    self.softirq_time +
    self.steal_time +
    self.guest_time +
    self.guest_nice_time
  }
}

