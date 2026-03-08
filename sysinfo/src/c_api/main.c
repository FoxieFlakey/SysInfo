#include <stdio.h>
#include <stddef.h>

#include "sysinfo.h"

static void print_opt(const char* name, const sysinfo_maybe_double* maybe) {
  if (maybe->coption_is_present)
    printf("Field '%s' present with %.2lf MiB\n", name, *sysinfo_coption_get(maybe));
  else
    printf("Field '%s' is not present\n", name);
}

int main() {
  struct sysinfo* sysinfo = sysinfo_new();
  sysinfo_update(sysinfo);
  
  const struct sysinfo_memory* memory = sysinfo_get_latest_memory_sample(sysinfo);
  if (memory == NULL) {
    fprintf(stderr, "ERROR: there is no recent memory sample\n");
    goto exit;
  }
  
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
  
exit:
  sysinfo_free(sysinfo);
}


