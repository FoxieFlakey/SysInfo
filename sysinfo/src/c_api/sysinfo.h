#ifndef UWU_CE835E26_8AE9_488D_856F_F024F8A73996_UWU
#define UWU_CE835E26_8AE9_488D_856F_F024F8A73996_UWU

#include <stddef.h>

// Opaque type from Rust side
struct sysinfo;

struct sysinfo* _Nonnull sysinfo_new();
void sysinfo_free(struct sysinfo* _Nonnull this);

void sysinfo_update(struct sysinfo* _Nonnull this);

#define SYSINFO_COPTION_TEMPLATE(name, T) struct name { \
    _Bool coption_is_present; \
    T coption_data; \
  }

#define SYSINFO_CVEC_TEMPLATE(name, T) \
  struct name { \
    size_t cvec_length; \
    size_t cvec_capacity; \
    T* _Nullable cvec_data; \
  }

struct sysinfo_cstring {
  size_t cstring_length;
  const char* _Nonnull cstring_string;
};


// -------------------------------------------------
// Memory metrics
// -------------------------------------------------

// Make the templates "real types"
// must use these instead of via template macro
// 
// This way would allow C++ methods to be usable
SYSINFO_COPTION_TEMPLATE(sysinfo_maybe_double, double);

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
  
  struct sysinfo_maybe_double lazy_free_kib;
  struct sysinfo_maybe_double direct_map_4k_kib;
  struct sysinfo_maybe_double direct_map_2m_kib;
  struct sysinfo_maybe_double direct_map_4m_kib;
  struct sysinfo_maybe_double direct_map_1g_kib;
  struct sysinfo_maybe_double hardware_corrupted_kib;
  struct sysinfo_maybe_double anon_huge_pages_kib;
  struct sysinfo_maybe_double shmem_huge_pages_kib;
  struct sysinfo_maybe_double shmem_mapped_huge_pages_kib;
  struct sysinfo_maybe_double huge_pages_total_kib;
  struct sysinfo_maybe_double huge_pages_free_kib;
  struct sysinfo_maybe_double huge_pages_reserved_kib;
  struct sysinfo_maybe_double huge_pages_surplus_kib;
  struct sysinfo_maybe_double huge_page_size_kib;
};

const struct sysinfo_memory* _Nullable sysinfo_get_latest_memory_sample(const struct sysinfo* _Nonnull this);

// -------------------------------------------------
// Swap metrics
// -------------------------------------------------

struct sysinfo_swapdev {
  struct sysinfo_cstring path;
  double size_kib;
  double used_kib;
};

SYSINFO_CVEC_TEMPLATE(sysinfo_swapdev_vec, struct sysinfo_swapdev);

struct sysinfo_swaps {
  double total_size_kib;
  double total_used_kib;
  struct sysinfo_swapdev_vec swapdevs;
};

const struct sysinfo_swaps* _Nullable sysinfo_get_latest_swap_sample(const struct sysinfo* _Nonnull this);

// Convenience macros & funcs, users can directly use the fields information instead
// doesn't have to use these macros for like C++ helper methods can avoids using
// this macro and work on the data directly

#define sysinfo_coption_get(this) ((this)->coption_is_present ? &(this)->coption_data : NULL)

#define sysinfo_cvec_len(this) ((this)->cvec_length)

// Returns pointer to the entry
static inline void* _Nullable sysinfo_cvec_get_impl(size_t len, size_t index, size_t dataLen, void* _Nullable dataPtr) {
  if (index >= len)
    return NULL;
  
  if (dataPtr == NULL)
    return NULL;
  
  return ((char*) dataPtr) + dataLen * index;
}

#define sysinfo_cvec_get(self, index) ((typeof((self)->cvec_data)) sysinfo_cvec_get_impl((self)->cvec_length, (index), sizeof(*(self)->cvec_data), (void*) (self)->cvec_data))

__attribute__((unused))
static inline size_t sysinfo_cstring_len(const struct sysinfo_cstring* _Nonnull this) {
  return this->cstring_length;
}

__attribute__((unused))
static inline const char* _Nonnull sysinfo_cstring_get(const struct sysinfo_cstring* _Nonnull this) {
  return this->cstring_string;
}

#endif
