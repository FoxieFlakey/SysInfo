use sysinfo::SysInfo;

fn main() {
  let mut sysinfo = SysInfo::new();
  sysinfo.update();
  
  let sample = sysinfo.cpu_usage.data.samples.front().unwrap().as_ref().unwrap();
  let swaps = sysinfo.swap_usage.data.samples.front().unwrap().as_ref().unwrap();
  let memory = sysinfo.memory_usage.data.samples.front().unwrap().as_ref().unwrap();
  
  println!("Utilization: {:5.2}%", sample.utilization * 100.0);
  println!("Frequency: {:5.2} Mhz", sample.frequency_khz / 1000.0);
  println!("Present CPU count: {}", sample.present);
  println!("Possible CPU count: {}", sample.possible);
  println!("Online CPU count: {}", sample.online);
  println!("Offline CPU count: {}", sample.offline);
  println!("Sockets:");
  for (i, socket) in sample.sockets.iter().enumerate() {
    println!(" - Socket {i}");
    println!("   Utilization: {:5.2}%", socket.utilization * 100.0);
    println!("   Frequency: {:5.2} Mhz", socket.frequency_khz / 1000.0);
    println!("   Clusters:");
    for (i, die) in socket.dies.iter().enumerate() {
      println!("   - Die {i}");
      println!("     Utilization: {:5.2}%", die.utilization * 100.0);
      println!("     Frequency: {:5.2} Mhz", die.frequency_khz / 1000.0);
      println!("     Clusters:");
      for (i, cluster) in die.clusters.iter().enumerate() {
        println!("     - Cluster {i}");
        println!("       Utilization: {:5.2}%", cluster.utilization * 100.0);
        println!("       Frequency: {:5.2} Mhz", cluster.frequency_khz / 1000.0);
        println!("       Cores:");
        for (i, core) in cluster.cores.iter().enumerate() {
          println!("       - Core {i}");
          println!("         Utilization: {:5.2}%", core.utilization * 100.0);
          println!("         Frequency: {:5.2} Mhz", core.frequency_khz / 1000.0);
          println!("         Threads:");
          for (i, thread) in core.threads.iter().enumerate() {
            println!("         - Hw thread {i}");
            println!("           Online percent: {:5.2}%", thread.online_percent * 100.0);
            println!("           Utilization: {:5.2}%", thread.utilization * 100.0);
            println!("           Frequency: {:5.2} Mhz", thread.frequency_khz / 1000.0);
          }
        }
      }
    }
  }
  
  println!("Swap in system:");
  println!("Total size: {:8.2} MiB", swaps.total_size_kib / 1024.0);
  println!("Total used: {:8.2} MiB", swaps.total_used_kib / 1024.0);
  println!("Swap devices:");
  for dev in &swaps.swaps {
    println!(" - Swap at {}", dev.path);
    println!("   Used: {:8.2} MiB", dev.used_kib / 1024.0);
    println!("   Size: {:8.2} MiB", dev.size_kib / 1024.0);
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
  
  println!("Total memory      : {:12.2} MiB", total);
  println!("Free              : {:12.2} MiB", free);
  println!("Used              : {:12.2} MiB", used);
  println!("Non cache/buffers : {:12.2} MiB", non_cache_or_buffer);
  println!("Cached            : {:12.2} MiB", cached);
  println!("Buffers           : {:12.2} MiB", buffers);
  println!("Shmem             : {:12.2} MiB", shmem);
  println!("Writeback         : {:12.2} MiB", writeback);
  println!("Dirty             : {:12.2} MiB", dirty);
  println!("Available         : {:12.2} MiB", available);
  println!("File mapped       : {:12.2} MiB", mapped);
  println!("Anonymous mem     : {:12.2} MiB", anonymous_memory);
  
  println!("Simpler summary of memory state:");
  println!("Used      : {:12.2} MiB", non_cache_or_buffer + shmem + buffers);
  println!("Cache     : {:12.2} MiB", cached);
  println!("Writeback : {:12.2} MiB (pending to written to disk)", writeback);
  println!("Available : {:12.2} MiB", available);
}

