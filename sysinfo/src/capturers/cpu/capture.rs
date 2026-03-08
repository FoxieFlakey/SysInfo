use std::{iter::Peekable, ops::Range, path::{Path, PathBuf}, time::Instant};

use arrayvec::ArrayString;
use indexmap::IndexMap;

use crate::{c_api::cvec::CVec, capturers::cpu::{CPU, Cluster, Core, Die, Socket, Thread, stat::ProcStat}, metric::Capturer, util};

pub struct CpuCapture {
  prev_stat: Option<(Instant, ProcStat)>
}

impl CpuCapture {
  pub fn new() -> Self {
    Self {
      prev_stat: None
    }
  }
}

impl Capturer for CpuCapture {
  type Sample = CPU;
  
  fn prep_capture(&mut self) {
    self.prev_stat = ProcStat::capture()
      .map(|x| (Instant::now(), x));
  }
  
  fn capture(&mut self) -> Option<Self::Sample> {
    let (start_time, prev_stat) = self.prev_stat.take()?;
    let current_stat = ProcStat::capture()?;
    let duration = start_time.elapsed().as_secs_f64();
    
    // There mutliple files under there
    // /sys/devices/system/cpu/online mean CPUs that is online
    // /sys/devices/system/cpu/offline mean CPUs that is offline
    // /sys/devices/system/cpu/present mean CPUs that is present
    // /sys/devices/system/cpu/possible mean CPUs that Linux can have at maximum
    //
    // Sizing of set is online + offline == present, and present <= possible
    // present also dictates how many /sys/devices/system/cpu/cpuX present (regardless of online/offline/present state)
    //
    // Source: google and kernel doc at https://www.kernel.org/doc/Documentation/ABI/stable/sysfs-devices-system-cpu
    
    let online_cpus = parse_list(&util::read_all(Path::new("/sys/devices/system/cpu/online")).ok()?)?;
    let offline_cpus = parse_list(&util::read_all(Path::new("/sys/devices/system/cpu/offline")).ok()?)?;
    let present_cpus = parse_list(&util::read_all(Path::new("/sys/devices/system/cpu/present")).ok()?)?;
    let possible_cpus = parse_list(&util::read_all(Path::new("/sys/devices/system/cpu/possible")).ok()?)?;
    
    // Maps physical_package_id to Socket
    let mut topology: IndexMap<u32,
      (
        Socket,
        // Maps die_id to Socket
        IndexMap<u32, (
          Die,
          // Maps cluster_id to Cluster
          IndexMap<u32, (
            Cluster,
            // Maps core_id to Core
            IndexMap<u32, Core>
          )>
        )>
      )
    > = IndexMap::new();
    
    for entry in &present_cpus {
      for id in entry.as_range() {
        let base = PathBuf::from(format!("/sys/devices/system/cpu/cpu{id}"));
        let cur_freq = read_integer(&base.join("cpufreq/scaling_cur_freq"))?;
        
        // On some CPU, the 'online' file didnt exists, it usually meant the cpu can't be taken offline
        // so its always online
        let is_online = read_integer(&base.join("online")).unwrap_or(1) != 0;
        
        let prev_time = prev_stat.cpus.get(id as usize)
          .cloned()
          .unwrap_or_default()
          .combine_all_active() as f64 * (1.0 / util::get_clock_tick_speed() as f64);
        
        let cur_time = current_stat.cpus.get(id as usize)
          .cloned()
          .unwrap_or_default()
          .combine_all_active() as f64 * (1.0 / util::get_clock_tick_speed() as f64);
        
        let time_active = cur_time - prev_time;
        let utilization = time_active / duration;
        
        let physical_package_id = read_integer(&base.join("topology/physical_package_id"))?;
        let die_id = read_integer(&base.join("topology/die_id"))?;
        let cluster_id = read_integer(&base.join("topology/cluster_id"))?;
        let core_id = read_integer(&base.join("topology/core_id"))?;
        
        let socket = topology
          .entry(physical_package_id)
          .or_insert_with(|| (
              Socket {
                frequency_khz: 0.0,
                utilization: 0.0,
                dies: CVec::new()
              },
              IndexMap::new()
            )
          );
        
        let die = socket.1
          .entry(die_id)
          .or_insert_with(|| (
              Die {
                frequency_khz: 0.0,
                utilization: 0.0,
                clusters: CVec::new()
              },
              IndexMap::new()
            )
          );
        
        let cluster = die.1
          .entry(cluster_id)
          .or_insert_with(|| (
              Cluster {
                frequency_khz: 0.0,
                utilization: 0.0,
                cores: CVec::new()
              },
              IndexMap::new()
            )
          );
        
        let core = cluster.1
          .entry(core_id)
          .or_insert_with(||
            Core {
              frequency_khz: 0.0,
              utilization: 0.0,
              threads: CVec::new()
            }
          );
        
        core.threads.push(Thread {
          frequency_khz: f64::from(cur_freq),
          online_percent: if is_online { 1.0 } else { 0.0 },
          utilization
        });
      }
    }
    
    let mut cpu = CPU {
      frequency_khz: 0.0,
      utilization: 0.0,
      
      present: count_cpus(&present_cpus).into(),
      possible: count_cpus(&possible_cpus).into(),
      online: count_cpus(&online_cpus).into(),
      offline: count_cpus(&offline_cpus).into(),
      
      sockets: CVec::new()
    };
    
    // Lol, nests :3
    for (_, (mut socket, dies)) in topology {
      for (_, (mut die, clusters)) in dies {
        for (_, (mut cluster, cores)) in clusters {
          for (_, core) in cores {
            cluster.cores.push(core);
          }
          die.clusters.push(cluster);
        }
        socket.dies.push(die);
      }
      cpu.sockets.push(socket);
    }
    
    cpu.sanify();
    
    Some(cpu)
  }
}

fn read_integer(path: &Path) -> Option<u32> {
  util::read_all(&path)
    .map(|x| {
      if x.starts_with("-") {
        // Negative is unacceptable
        None
      } else {
        let mut num = ArrayString::<10>::new();
        for chr in x.chars() {
          match chr {
            '0'..='9' => {
              if num.try_push(chr).is_err() {
                return None;
              }
            }                  
            '\n' => break,
            _ => return None
          }
        }
        
        u32::from_str_radix(&num, 10).ok()
      }
    })
    .ok()
    .flatten()
}

fn parse_list(cpu_list: &str) -> Option<Vec<CPUEntry>> {
  let mut iter = cpu_list.chars().fuse().peekable();
  let mut list = Vec::new();
  
  // Its empty entry
  if let Some('\n') = iter.peek() {
    return Some(list);
  }
  
  loop {
    list.push(CPUEntry::parse(&mut iter)?);
    
    match iter.next() {
      Some(',') => (),
      Some('\n') => break,
      _ => return None
    }
  }
  
  Some(list)
}

fn count_cpus(list: &[CPUEntry]) -> u32 {
  list.iter()
    .fold(0, |acc, entry| {
      let num = match entry {
        CPUEntry::Range(start, end) => *end - *start,
        CPUEntry::Single(x) => *x
      };
      
      num + acc
    })
}

// In Linux kernel the list is a "CPU" although practically
// its more of threads list
#[derive(Clone, Copy)]
enum CPUEntry {
  // the 0-3, format, but the .1 is exclusive as its idiomatic in rust
  // so here its 0 and 4
  Range(u32, u32),
  // singular 2
  Single(u32)
}

impl CPUEntry {
  fn as_range(self) -> Range<u32> {
    match self {
      CPUEntry::Range(start, end) => start..end,
      CPUEntry::Single(x) => x..(x + 1)
    }
  }
  
  fn parse<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> Option<CPUEntry> {
    // Because using 32-bit to refer the CPU, in decimal maximum
    // count is 10 characters 4294967296
    let mut first_number = ArrayString::<12>::new();
    
    // Try fetch first number
    loop {
      match iter.peek() {
        Some(&'-') => {
          // Cpu entry is invalid, either empty string like ""
          // or starts with dash like "-32"
          if first_number.len() == 0 {
            return None;
          }
          
          break;
        },
        // EOL reached
        Some(&'\n') => break,
        Some(&c @ '0'..='9') => {
          if first_number.try_push(c).is_err() {
            // The number is too big to fit in u32, lets
            // fail the parse
            return None;
          }
          
          // Move forward the iteration
          iter.next();
        }
        
        // Unknown character/unexpected EOF
        None | Some(_) => return None
      }
    }
    
    let Ok(lhs) = u32::from_str_radix(&first_number, 10) else {
        return None;
      };
    
    if let Some('-') = iter.peek() {
      // Consume the '-' seperator
      iter.next();
      
      // Its a range lets find second number
      let mut second_number = ArrayString::<10>::new();
      
      loop {
        match iter.peek() {
          // Read all the digits
          Some(&chr @ '0'..='9') => {
            if second_number.try_push(chr).is_err() {
              // Second number is too long
              return None;
            }
            
            // Consume the character
            iter.next();
          }
          
          // Digits to read ran out
          None | Some(_) => break
        }
      }
      
      let rhs = u32::from_str_radix(&second_number, 10).ok()?;
      if rhs <= lhs {
        // Invalid end
        return None;
      }
      
      Some(CPUEntry::Range(lhs, rhs + 1))
    } else {
      Some(CPUEntry::Single(lhs))
    }
  }
}


