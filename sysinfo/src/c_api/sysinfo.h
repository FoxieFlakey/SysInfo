#ifndef UWU_CE835E26_8AE9_488D_856F_F024F8A73996_UWU
#define UWU_CE835E26_8AE9_488D_856F_F024F8A73996_UWU

// Opaque type from Rust side
struct sysinfo;

struct sysinfo* _Nonnull sysinfo_new();
void sysinfo_free(struct sysinfo* _Nonnull this);

void sysinfo_update(struct sysinfo* _Nonnull this);

// Get metric for a given resource
struct sysinfo_memory {
  double mem_total_kib;
  double mem_free_kib;
  double mem_available_kib;
  double cached_kib;
  double buffers_kib;
  double swap_cached_kib;
  double active_kib;
  double inactive_kib;
  double unevictable_kib;
  double mlocked_kib;
  double dirty_kib;
  double writeback_kib;
  double anon_pages_kib;
  double mapped_kib;
  double shmem_kib;
  double kreclaimable_kib;
  double slab_kib;
  double slab_reclaimable_kib;
  double slab_unreclaimable_kib;
  double kernel_stacks_kib;
  double page_tables_kib;
  double bounce_kib;
  double writeback_temp_kib;
  
  // Note there additional data but it uses Rust type
  // so i intentionally exclude it for now, till
  // I make C compatible "Option" types
};

struct sysinfo_memory* _Nullable sysinfo_get_latest_memory_sample(const struct sysinfo* _Nonnull this);

#endif
