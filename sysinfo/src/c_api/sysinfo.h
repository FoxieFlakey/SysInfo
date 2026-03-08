#ifndef UWU_CE835E26_8AE9_488D_856F_F024F8A73996_UWU
#define UWU_CE835E26_8AE9_488D_856F_F024F8A73996_UWU

#include <stddef.h>

// Opaque type from Rust side
struct sysinfo;

struct sysinfo* _Nonnull sysinfo_new();
void sysinfo_free(struct sysinfo* _Nonnull this);

void sysinfo_update(struct sysinfo* _Nonnull this);

#define SYSINFO_COPTION_TEMPLATE(T) struct { \
    _Bool coption_is_present; \
    T coption_data; \
  }

#define sysinfo_coption_get(this) ((this)->coption_is_present ? &(this)->coption_data : NULL)

#define SYSINFO_CVEC_TEMPLATE(T) \
  struct { \
    size_t cvec_length; \
    size_t cvec_capacity; \
    T* cvec_data; \
  }

#define sysinfo_cvec_len(this) ((self)->cvec_length)

// Returns pointer to the entry
#define sysinfo_cvec_index(this, idx) (&(self)->cvec_data[(idx)])

// Make the templates "real types"
// must use these instead of via template macro
// 
// This way would allow C++ methods to be usable
typedef SYSINFO_COPTION_TEMPLATE(double) sysinfo_maybe_double;

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
  
  sysinfo_maybe_double lazy_free_kib;
  sysinfo_maybe_double direct_map_4k_kib;
  sysinfo_maybe_double direct_map_2m_kib;
  sysinfo_maybe_double direct_map_4m_kib;
  sysinfo_maybe_double direct_map_1g_kib;
  sysinfo_maybe_double hardware_corrupted_kib;
  sysinfo_maybe_double anon_huge_pages_kib;
  sysinfo_maybe_double shmem_huge_pages_kib;
  sysinfo_maybe_double shmem_mapped_huge_pages_kib;
  sysinfo_maybe_double huge_pages_total_kib;
  sysinfo_maybe_double huge_pages_free_kib;
  sysinfo_maybe_double huge_pages_reserved_kib;
  sysinfo_maybe_double huge_pages_surplus_kib;
  sysinfo_maybe_double huge_page_size_kib;
};

struct sysinfo_memory* _Nullable sysinfo_get_latest_memory_sample(const struct sysinfo* _Nonnull this);

#endif
