#include <stdio.h>
#include <stddef.h>
#include <stdlib.h>

#include "sysinfo.h"

static void print_opt(const char* name, const struct sysinfo_maybe_double* maybe) {
  if (maybe->coption_is_present)
    printf("Field '%s' present with %.2lf MiB\n", name, *sysinfo_coption_get(maybe));
  else
    printf("Field '%s' is not present\n", name);
}

int main() {
  struct sysinfo* sysinfo = sysinfo_new();
  sysinfo_update(sysinfo);
  
  const struct sysinfo_cpu* sample = sysinfo_get_latest_cpu_sample(sysinfo);
  if (sample == NULL) {
    fprintf(stderr, "ERROR: there is no recent cpu sample\n");
    goto exit;
  }
  
  const struct sysinfo_memory* memory = sysinfo_get_latest_memory_sample(sysinfo);
  if (memory == NULL) {
    fprintf(stderr, "ERROR: there is no recent memory sample\n");
    goto exit;
  }
  
  const struct sysinfo_swaps* swap = sysinfo_get_latest_swap_sample(sysinfo);
  if (swap == NULL) {
    fprintf(stderr, "ERROR: there is no recent swap sample\n");
    goto exit;
  }
  
  const struct sysinfo_loadavg* loadavg = sysinfo_get_latest_loadavg_sample(sysinfo);
  if (loadavg == NULL) {
    fprintf(stderr, "ERROR: there is no recent loadavg sample\n");
    goto exit;
  }
  
  size_t coreCount = 0;
  
  printf("Utilization        : %5.2lf%%\n", sample->utilization * 100.0);
  printf("Frequency          : %5.2lf Mhz\n", sample->frequency_khz / 1000.0);
  printf("Present CPU count  : %5.2lf\n", sample->present);
  printf("Possible CPU count : %5.2lf\n", sample->possible);
  printf("Online CPU count   : %5.2lf\n", sample->online);
  printf("Offline CPU count  : %5.2lf\n", sample->offline);
  printf("Sockets:\n");
  for (size_t i = 0; i < sysinfo_cvec_len(&sample->sockets); i++) {
    const struct sysinfo_cpu_socket* socket = sysinfo_cvec_get(&sample->sockets, i);
    printf(" - Socket %zu\n", i);
    printf("   Utilization : %5.2lf%%\n", socket->utilization * 100.0);
    printf("   Frequency   : %5.2lf Mhz\n", socket->frequency_khz / 1000.0);
    printf("   Clusters:\n");
    
    for (size_t i = 0; i < sysinfo_cvec_len(&socket->dies); i++) {
      const struct sysinfo_cpu_die* die = sysinfo_cvec_get(&socket->dies, i);
      printf("   - Die %zu\n", i);
      printf("     Utilization : %5.2lf%%\n", die->utilization * 100.0);
      printf("     Frequency   : %5.2lf Mhz\n", die->frequency_khz / 1000.0);
      printf("     Clusters:\n");
      for (size_t i = 0; i < sysinfo_cvec_len(&die->clusters); i++) {
        const struct sysinfo_cpu_cluster* cluster = sysinfo_cvec_get(&die->clusters, i);
        printf("     - Cluster %zu\n", i);
        printf("       Utilization : %5.2lf%%\n", cluster->utilization * 100.0);
        printf("       Frequency   : %5.2lf Mhz\n", cluster->frequency_khz / 1000.0);
        printf("       Cores:\n");
        for (size_t i = 0; i < sysinfo_cvec_len(&cluster->cores); i++) {
          const struct sysinfo_cpu_core* core = sysinfo_cvec_get(&cluster->cores, i);
          printf("       - Core %zu\n", i);
          printf("         Utilization : %5.2lf%%\n", core->utilization * 100.0);
          printf("         Frequency   : %5.2lf Mhz\n", core->frequency_khz / 1000.0);
          printf("         Threads:\n");
          for (size_t i = 0; i < sysinfo_cvec_len(&core->threads); i++) {
            const struct sysinfo_cpu_thread* thread = sysinfo_cvec_get(&core->threads, i);
            printf("         - Thread %zu\n", i);
            printf("           Utilization : %5.2lf%%\n", thread->utilization * 100.0);
            printf("           Frequency   : %5.2lf Mhz\n", thread->frequency_khz / 1000.0);
          }
        }
        
        coreCount += sysinfo_cvec_len(&cluster->cores);
      }
    }
  }
  
  const struct sysinfo_cpu_core** cores = calloc(coreCount, sizeof(*cores));
  if (!cores) {
    printf("ERROR: Cannot allocate buffer to store list of cores\n");
    goto exit;
  }
  
  size_t currentIndex = 0;
  for (size_t i = 0; i < sysinfo_cvec_len(&sample->sockets); i++) {
    const struct sysinfo_cpu_socket* socket = sysinfo_cvec_get(&sample->sockets, i);
    for (size_t i = 0; i < sysinfo_cvec_len(&socket->dies); i++) {
      const struct sysinfo_cpu_die* die = sysinfo_cvec_get(&socket->dies, i);
      for (size_t i = 0; i < sysinfo_cvec_len(&die->clusters); i++) {
        const struct sysinfo_cpu_cluster* cluster = sysinfo_cvec_get(&die->clusters, i);
        for (size_t i = 0; i < sysinfo_cvec_len(&cluster->cores); i++) {
          const struct sysinfo_cpu_core* core = sysinfo_cvec_get(&cluster->cores, i);
          
          if (currentIndex >= coreCount) {
            free(cores);
            printf("ERROR: There suddenly new entry in amount of cores! (MUST NOT HAPPEN)\n");
            goto exit;
          }
          
          cores[currentIndex] = core;
          currentIndex++;
        }
      }
    }
  }
  
  printf("Simplified report: only the physical cores reported, not including hyperthreads if there any\n");
  for (size_t i = 0; i < coreCount; i++) {
    const struct sysinfo_cpu_core* core = cores[i];
    printf("CPU %zu %6.2lf%% usage @ %8.2lf Mhz\n", i, core->utilization * 100.0, core->frequency_khz / 1000.0);
  }
  
  free(cores);
  
  double used = (memory->mem_total_kib - memory->mem_free_kib) / 1024.0;
  double total = memory->mem_total_kib / 1024.0;
  double non_cache_or_buffer = used - (memory->buffers_kib + memory->cached_kib) / 1024.0;
  double cached = (memory->cached_kib + memory->slab_reclaimable_kib - memory->shmem_kib) / 1024.0;
  double buffers = memory->buffers_kib / 1024.0;
  double free = memory->mem_free_kib / 1024.0;
  double shmem = memory->shmem_kib / 1024.0;
  double writeback = memory->writeback_kib / 1024.0;
  double mapped = memory->mapped_kib / 1024.0;
  double dirty = memory->dirty_kib / 1024.0;
  double available = memory->mem_available_kib / 1024.0;
  double anonymous_memory = memory->anon_pages_kib / 1024.0;
  
  printf("This is same output as the one from Rust's test_uwu crate using same API but with C meow meow~ :3\n");
  
  printf("Total memory      : %12.2lf MiB\n", total);
  printf("Free              : %12.2lf MiB\n", free);
  printf("Used              : %12.2lf MiB\n", used);
  printf("Non cache/buffers : %12.2lf MiB\n", non_cache_or_buffer);
  printf("Cached            : %12.2lf MiB\n", cached);
  printf("Buffers           : %12.2lf MiB\n", buffers);
  printf("Shmem             : %12.2lf MiB\n", shmem);
  printf("Writeback         : %12.2lf MiB\n", writeback);
  printf("Dirty             : %12.2lf MiB\n", dirty);
  printf("Available         : %12.2lf MiB\n", available);
  printf("File mapped       : %12.2lf MiB\n", mapped);
  printf("Anonymous mem     : %12.2lf MiB\n", anonymous_memory);
  
  printf("Simpler summary of memory state:\n");
  printf("Used      : %12.2lf MiB\n", non_cache_or_buffer + shmem + buffers);
  printf("Cache     : %12.2lf MiB\n", cached);
  printf("Writeback : %12.2lf MiB (pending to written to disk)\n", writeback);
  printf("Available : %12.2lf MiB\n", available);
  
  printf("\nOptional fields:\n");
  
  print_opt("lazy_free_kib", &memory->lazy_free_kib);
  print_opt("direct_map_4k_kib", &memory->direct_map_4k_kib);
  print_opt("direct_map_2m_kib", &memory->direct_map_2m_kib);
  print_opt("direct_map_4m_kib", &memory->direct_map_4m_kib);
  print_opt("direct_map_1g_kib", &memory->direct_map_1g_kib);
  print_opt("hardware_corrupted_kib", &memory->hardware_corrupted_kib);
  print_opt("anon_huge_pages_kib", &memory->anon_huge_pages_kib);
  print_opt("shmem_huge_pages_kib", &memory->shmem_huge_pages_kib);
  print_opt("shmem_mapped_huge_pages_kib", &memory->shmem_mapped_huge_pages_kib);
  print_opt("huge_pages_total_kib", &memory->huge_pages_total_kib);
  print_opt("huge_pages_free_kib", &memory->huge_pages_free_kib);
  print_opt("huge_pages_reserved_kib", &memory->huge_pages_reserved_kib);
  print_opt("huge_pages_surplus_kib", &memory->huge_pages_surplus_kib);
  print_opt("huge_page_size_kib", &memory->huge_page_size_kib);
  
  printf("Swap in system:\n");
  printf("Total size: %8.2lf MiB\n", swap->total_size_kib / 1024.0);
  printf("Total used: %8.2lf MiB\n", swap->total_used_kib / 1024.0);
  printf("Swap devices:\n");
  for (size_t i = 0; i < sysinfo_cvec_len(&swap->swapdevs); i++) {
    const struct sysinfo_swapdev* dev = sysinfo_cvec_get(&swap->swapdevs, i);
    printf(" - Swap at %s\n", sysinfo_cstring_get(&dev->path));
    printf("   Used: %8.2lf MiB\n", dev->used_kib / 1024.0);
    printf("   Size: %8.2lf MiB\n", dev->size_kib / 1024.0);
  }
  
  printf("System loadavg:\n");
  printf("1  minute     : %8.2lf\n", loadavg->load_1m);
  printf("5  minute     : %8.2lf\n", loadavg->load_5m);
  printf("15 minute     : %8.2lf\n", loadavg->load_15m);
  printf("Runnable tasks: %8.2lf tasks\n", loadavg->runnable_task_count);
  printf("Task count    : %8.2lf tasks\n", loadavg->task_count);
exit:
  sysinfo_free(sysinfo);
}


