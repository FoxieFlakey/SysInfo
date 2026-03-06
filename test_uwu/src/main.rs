use sysinfo::SysInfo;

fn main() {
  let mut sysinfo = SysInfo::new();
  sysinfo.update();
  
  let sample = sysinfo.cpu_usage.data.samples.front().unwrap().as_ref().unwrap();
  let swaps = sysinfo.swap_usage.data.samples.front().unwrap().as_ref().unwrap();
  
  println!("Utilization: {}%", sample.utilization * 100.0);
  println!("Frequency: {} Mhz", sample.frequency_khz / 1000.0);
  println!("Present CPU count: {}", sample.present);
  println!("Possible CPU count: {}", sample.possible);
  println!("Online CPU count: {}", sample.online);
  println!("Offline CPU count: {}", sample.offline);
  println!("Sockets:");
  for (i, socket) in sample.sockets.iter().enumerate() {
    println!(" - Socket {i}");
    println!("   Utilization: {:5.2}%", socket.utilization * 100.0);
    println!("   Frequency: {} Mhz", socket.frequency_khz / 1000.0);
    println!("   Clusters:");
    for (i, die) in socket.dies.iter().enumerate() {
      println!("   - Die {i}");
      println!("     Utilization: {:5.2}%", die.utilization * 100.0);
      println!("     Frequency: {} Mhz", die.frequency_khz / 1000.0);
      println!("     Clusters:");
      for (i, cluster) in die.clusters.iter().enumerate() {
        println!("     - Cluster {i}");
        println!("       Utilization: {:5.2}%", cluster.utilization * 100.0);
        println!("       Frequency: {} Mhz", cluster.frequency_khz / 1000.0);
        println!("       Cores:");
        for (i, core) in cluster.cores.iter().enumerate() {
          println!("       - Core {i}");
          println!("         Utilization: {:5.2}%", core.utilization * 100.0);
          println!("         Frequency: {} Mhz", core.frequency_khz / 1000.0);
          println!("         Threads:");
          for (i, thread) in core.threads.iter().enumerate() {
            println!("         - Hw thread {i}");
            println!("           Online percent: {:5.2}%", thread.online_percent * 100.0);
            println!("           Utilization: {:5.2}%", thread.utilization * 100.0);
            println!("           Frequency: {} Mhz", thread.frequency_khz / 1000.0);
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
}

