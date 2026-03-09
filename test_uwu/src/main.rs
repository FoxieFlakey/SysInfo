use sysinfo::{SysInfo, c_api::{cvec::CVec, option::COption}};

fn main() {
  let mut sysinfo = SysInfo::new();
  sysinfo.update();
  
  let sample = sysinfo.cpu_usage.data.samples.front().unwrap().as_ref().unwrap();
  let swaps = sysinfo.swap_usage.data.samples.front().unwrap().as_ref().unwrap();
  let memory = sysinfo.memory_usage.data.samples.front().unwrap().as_ref().unwrap();
  let loadavg = sysinfo.loadavg.data.samples.front().unwrap().as_ref().unwrap();
  
  println!("Utilization: {:6.2}%", sample.utilization * 100.0);
  println!("Frequency: {:6.2} Mhz", sample.frequency_khz / 1000.0);
  println!("Present CPU count: {}", sample.present);
  println!("Possible CPU count: {}", sample.possible);
  println!("Online CPU count: {}", sample.online);
  println!("Offline CPU count: {}", sample.offline);
  println!("Sockets:");
  for (i, socket) in sample.sockets.iter().enumerate() {
    println!(" - Socket {i}");
    println!("   Utilization: {:6.2}%", socket.utilization * 100.0);
    println!("   Frequency: {:6.2} Mhz", socket.frequency_khz / 1000.0);
    println!("   Clusters:");
    for (i, die) in socket.dies.iter().enumerate() {
      println!("   - Die {i}");
      println!("     Utilization: {:6.2}%", die.utilization * 100.0);
      println!("     Frequency: {:6.2} Mhz", die.frequency_khz / 1000.0);
      println!("     Clusters:");
      for (i, cluster) in die.clusters.iter().enumerate() {
        println!("     - Cluster {i}");
        println!("       Utilization: {:6.2}%", cluster.utilization * 100.0);
        println!("       Frequency: {:6.2} Mhz", cluster.frequency_khz / 1000.0);
        println!("       Cores:");
        for (i, core) in cluster.cores.iter().enumerate() {
          println!("       - Core {i}");
          println!("         Utilization: {:6.2}%", core.utilization * 100.0);
          println!("         Frequency: {:6.2} Mhz", core.frequency_khz / 1000.0);
          println!("         Threads:");
          for (i, thread) in core.threads.iter().enumerate() {
            println!("         - Hw thread {i}");
            println!("           Online percent: {:6.2}%", thread.online_percent * 100.0);
            println!("           Utilization: {:6.2}%", thread.utilization * 100.0);
            println!("           Frequency: {:6.2} Mhz", thread.frequency_khz / 1000.0);
          }
        }
      }
    }
  }
  
  let cores = sample.sockets
    .iter()
    .map(|x| CVec::iter(&x.dies))
    .flatten()
    .map(|x| CVec::iter(&x.clusters))
    .flatten()
    .map(|x| CVec::iter(&x.cores))
    .flatten()
    .collect::<Vec<_>>();
  
  println!("Simplified report: only the physical cores reported, not including hyperthreads if there any");
  for (i, core) in cores.iter().enumerate() {
    println!("CPU {i} {:6.2}f% usage @ {:6.2} Mhz", core.utilization * 100.0, core.frequency_khz / 1000.0);
  }
  
  println!("Swap in system:");
  println!("Total size: {:9.2} MiB", swaps.total_size_kib / 1024.0);
  println!("Total used: {:9.2} MiB", swaps.total_used_kib / 1024.0);
  println!("Swap devices:");
  for dev in &swaps.swaps {
    println!(" - Swap at {}", dev.path);
    println!("   Used: {:9.2} MiB", dev.used_kib / 1024.0);
    println!("   Size: {:9.2} MiB", dev.size_kib / 1024.0);
  }
  
  println!("System's memory state:");
  let used = (memory.mem_total_kib - memory.mem_free_kib) / 1024.0;
  let total = memory.mem_total_kib / 1024.0;
  let non_cache_or_buffer = used - (memory.buffers_kib + memory.cached_kib) / 1024.0;
  let cached = (memory.cached_kib + memory.slab_reclaimable_kib - memory.shmem_kib) / 1024.0;
  let buffers = memory.buffers_kib / 1024.0;
  let free = memory.mem_free_kib / 1024.0;
  let shmem = memory.shmem_kib / 1024.0;
  let writeback = memory.writeback_kib / 1024.0;
  let mapped = memory.mapped_kib / 1024.0;024.0;
  let dirty = memory.dirty_kib / 1024.0;
  let available = memory.mem_available_kib / 1024.0;
  let anonymous_memory = memory.anon_pages_kib / 1024.0;
  
  println!("Total memory      : {:13.2} MiB", total);
  println!("Free              : {:13.2} MiB", free);
  println!("Used              : {:13.2} MiB", used);
  println!("Non cache/buffers : {:13.2} MiB", non_cache_or_buffer);
  println!("Cached            : {:13.2} MiB", cached);
  println!("Buffers           : {:13.2} MiB", buffers);
  println!("Shmem             : {:13.2} MiB", shmem);
  println!("Writeback         : {:13.2} MiB", writeback);
  println!("Dirty             : {:13.2} MiB", dirty);
  println!("Available         : {:13.2} MiB", available);
  println!("File mapped       : {:13.2} MiB", mapped);
  println!("Anonymous mem     : {:13.2} MiB", anonymous_memory);
  
  println!("Simpler summary of memory state:");
  println!("Used      : {:13.2} MiB", non_cache_or_buffer + shmem + buffers);
  println!("Cache     : {:13.2} MiB", cached);
  println!("Writeback : {:13.2} MiB (pending to written to disk)", writeback);
  println!("Available : {:13.2} MiB", available);
  
  fn print_opt(name: &str, opt: &COption<f64>) {
    match opt.as_ref().into_opt() {
      Some(x) => println!("Field '{name}' present with {x:.2} MiB"),
      None => println!("Field '{name}' not present")
    }
  }
  
  print_opt("lazy_free_kib", &memory.lazy_free_kib);
  print_opt("direct_map_4k_kib", &memory.direct_map_4k_kib);
  print_opt("direct_map_2m_kib", &memory.direct_map_2m_kib);
  print_opt("direct_map_4m_kib", &memory.direct_map_4m_kib);
  print_opt("direct_map_1g_kib", &memory.direct_map_1g_kib);
  print_opt("hardware_corrupted_kib", &memory.hardware_corrupted_kib);
  print_opt("anon_huge_pages_kib", &memory.anon_huge_pages_kib);
  print_opt("shmem_huge_pages_kib", &memory.shmem_huge_pages_kib);
  print_opt("shmem_mapped_huge_pages_kib", &memory.shmem_mapped_huge_pages_kib);
  print_opt("huge_pages_total_kib", &memory.huge_pages_total_kib);
  print_opt("huge_pages_free_kib", &memory.huge_pages_free_kib);
  print_opt("huge_pages_reserved_kib", &memory.huge_pages_reserved_kib);
  print_opt("huge_pages_surplus_kib", &memory.huge_pages_surplus_kib);
  print_opt("huge_page_size_kib", &memory.huge_page_size_kib);
  
  println!("System loadavg:");
  println!("1  minute     : {:9.2}", loadavg.load_1m);
  println!("5  minute     : {:9.2}", loadavg.load_5m);
  println!("15 minute     : {:9.2}", loadavg.load_15m);
  println!("Runnable tasks: {:9.2} tasks", loadavg.runnable_task_count);
  println!("Task count    : {:9.2} tasks", loadavg.task_count);
}

