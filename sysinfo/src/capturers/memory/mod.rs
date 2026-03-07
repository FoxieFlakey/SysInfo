// See https://man7.org/linux/man-pages/man5/proc_meminfo.5.html
//
// For majority of use,
// mem_total_kib, mem_free_kib and mem_available_kib are enough
//
// For bit more detailed like htop
// See https://stackoverflow.com/questions/41224738/how-to-calculate-system-memory-usage-from-proc-meminfo-like-htop
//
// > used = mem_total_kib - mem_free_kib
// > non cache/buffer = used - (buffers_kib + cached_kib)
// > cached = cached_kib + slab_reclaimable_kib - shmem_kib
//
// Personally i would add a new entry for shmem and mmaped stuffs and moves them
// out from "used"
//
// > used = mem_total_kib - mem_free_kib - shmem_kibs - mapped_kib
// > non cache/buffer = used - (buffers_kib + cached_kib)
// > cached = cached_kib + slab_reclaimable_kib - shmem_kib
// > shmem = shmem_kib
// > file_mapped = mapped_kib

use std::ops::{AddAssign, DivAssign, SubAssign};

mod capturer;
pub use capturer::MemoryCapturer;

use crate::metric::Sample;

#[derive(Clone)]
#[repr(C)]
pub struct Memory {
  pub mem_total_kib: f64,
  pub mem_free_kib: f64,
  pub mem_available_kib: f64,
  pub cached_kib: f64,
  pub buffers_kib: f64,
  pub swap_cached_kib: f64,
  pub active_kib: f64,
  pub inactive_kib: f64,
  pub unevictable_kib: f64,
  pub mlocked_kib: f64,
  pub dirty_kib: f64,
  pub writeback_kib: f64,
  pub anon_pages_kib: f64,
  pub mapped_kib: f64,
  pub shmem_kib: f64,
  pub kreclaimable_kib: f64,
  pub slab_kib: f64,
  pub slab_reclaimable_kib: f64,
  pub slab_unreclaimable_kib: f64,
  pub kernel_stacks_kib: f64,
  pub page_tables_kib: f64,
  pub bounce_kib: f64,
  pub writeback_temp_kib: f64,
  
  // Things that is optional
  pub lazy_free_kib: Option<f64>,
  pub direct_map_4k_kib: Option<f64>,
  pub direct_map_2m_kib: Option<f64>,
  pub direct_map_4m_kib: Option<f64>,
  pub direct_map_1g_kib: Option<f64>,
  pub hardware_corrupted_kib: Option<f64>,
  pub anon_huge_pages_kib: Option<f64>,
  pub shmem_huge_pages_kib: Option<f64>,
  pub shmem_mapped_huge_pages_kib: Option<f64>,
  pub huge_pages_total_kib: Option<f64>,
  pub huge_pages_free_kib: Option<f64>,
  pub huge_pages_reserved_kib: Option<f64>,
  pub huge_pages_surplus_kib: Option<f64>,
  pub huge_page_size_kib: Option<f64>
}

impl<'a> AddAssign<&'a Self> for Memory {
  fn add_assign(&mut self, rhs: &'a Self) {
    self.mem_total_kib += rhs.mem_total_kib;
    self.mem_free_kib += rhs.mem_free_kib;
    self.mem_available_kib += rhs.mem_available_kib;
    self.cached_kib += rhs.cached_kib;
    self.buffers_kib += rhs.buffers_kib;
    self.swap_cached_kib += rhs.swap_cached_kib;
    self.active_kib += rhs.active_kib;
    self.inactive_kib += rhs.inactive_kib;
    self.unevictable_kib += rhs.unevictable_kib;
    self.mlocked_kib += rhs.mlocked_kib;
    self.dirty_kib += rhs.dirty_kib;
    self.writeback_kib += rhs.writeback_kib;
    self.anon_pages_kib += rhs.anon_pages_kib;
    self.mapped_kib += rhs.mapped_kib;
    self.shmem_kib += rhs.shmem_kib;
    self.kreclaimable_kib += rhs.kreclaimable_kib;
    self.slab_kib += rhs.slab_kib;
    self.slab_reclaimable_kib += rhs.slab_reclaimable_kib;
    self.slab_unreclaimable_kib += rhs.slab_unreclaimable_kib;
    self.kernel_stacks_kib += rhs.kernel_stacks_kib;
    self.page_tables_kib += rhs.page_tables_kib;
    self.bounce_kib += rhs.bounce_kib;
    self.writeback_temp_kib += rhs.writeback_temp_kib;
    
    self.lazy_free_kib.iter_mut()
      .zip(rhs.lazy_free_kib)
      .for_each(|(lhs, rhs)| {
        *lhs += rhs;
      });
    self.direct_map_4k_kib.iter_mut()
      .zip(rhs.direct_map_4k_kib)
      .for_each(|(lhs, rhs)| {
        *lhs += rhs;
      });
    self.direct_map_2m_kib.iter_mut()
      .zip(rhs.direct_map_2m_kib)
      .for_each(|(lhs, rhs)| {
        *lhs += rhs;
      });
    self.direct_map_4m_kib.iter_mut()
      .zip(rhs.direct_map_4m_kib)
      .for_each(|(lhs, rhs)| {
        *lhs += rhs;
      });
    self.direct_map_1g_kib.iter_mut()
      .zip(rhs.direct_map_1g_kib)
      .for_each(|(lhs, rhs)| {
        *lhs += rhs;
      });
    self.hardware_corrupted_kib.iter_mut()
      .zip(rhs.hardware_corrupted_kib)
      .for_each(|(lhs, rhs)| {
        *lhs += rhs;
      });
    self.anon_huge_pages_kib.iter_mut()
      .zip(rhs.anon_huge_pages_kib)
      .for_each(|(lhs, rhs)| {
        *lhs += rhs;
      });
    self.shmem_huge_pages_kib.iter_mut()
      .zip(rhs.shmem_huge_pages_kib)
      .for_each(|(lhs, rhs)| {
        *lhs += rhs;
      });
    self.shmem_mapped_huge_pages_kib.iter_mut()
      .zip(rhs.shmem_mapped_huge_pages_kib)
      .for_each(|(lhs, rhs)| {
        *lhs += rhs;
      });
    self.huge_pages_total_kib.iter_mut()
      .zip(rhs.huge_pages_total_kib)
      .for_each(|(lhs, rhs)| {
        *lhs += rhs;
      });
    self.huge_pages_free_kib.iter_mut()
      .zip(rhs.huge_pages_free_kib)
      .for_each(|(lhs, rhs)| {
        *lhs += rhs;
      });
    self.huge_pages_reserved_kib.iter_mut()
      .zip(rhs.huge_pages_reserved_kib)
      .for_each(|(lhs, rhs)| {
        *lhs += rhs;
      });
    self.huge_pages_surplus_kib.iter_mut()
      .zip(rhs.huge_pages_surplus_kib)
      .for_each(|(lhs, rhs)| {
        *lhs += rhs;
      });
    self.huge_page_size_kib.iter_mut()
      .zip(rhs.huge_page_size_kib)
      .for_each(|(lhs, rhs)| {
        *lhs += rhs;
      });
  }
}

impl<'a> SubAssign<&'a Self> for Memory {
  fn sub_assign(&mut self, rhs: &'a Self) {
    self.mem_total_kib -= rhs.mem_total_kib;
    self.mem_free_kib -= rhs.mem_free_kib;
    self.mem_available_kib -= rhs.mem_available_kib;
    self.cached_kib -= rhs.cached_kib;
    self.buffers_kib -= rhs.buffers_kib;
    self.swap_cached_kib -= rhs.swap_cached_kib;
    self.active_kib -= rhs.active_kib;
    self.inactive_kib -= rhs.inactive_kib;
    self.unevictable_kib -= rhs.unevictable_kib;
    self.mlocked_kib -= rhs.mlocked_kib;
    self.dirty_kib -= rhs.dirty_kib;
    self.writeback_kib -= rhs.writeback_kib;
    self.anon_pages_kib -= rhs.anon_pages_kib;
    self.mapped_kib -= rhs.mapped_kib;
    self.shmem_kib -= rhs.shmem_kib;
    self.kreclaimable_kib -= rhs.kreclaimable_kib;
    self.slab_kib -= rhs.slab_kib;
    self.slab_reclaimable_kib -= rhs.slab_reclaimable_kib;
    self.slab_unreclaimable_kib -= rhs.slab_unreclaimable_kib;
    self.kernel_stacks_kib -= rhs.kernel_stacks_kib;
    self.page_tables_kib -= rhs.page_tables_kib;
    self.bounce_kib -= rhs.bounce_kib;
    self.writeback_temp_kib -= rhs.writeback_temp_kib;
    
    self.lazy_free_kib.iter_mut()
      .zip(rhs.lazy_free_kib)
      .for_each(|(lhs, rhs)| {
        *lhs -= rhs;
      });
    self.direct_map_4k_kib.iter_mut()
      .zip(rhs.direct_map_4k_kib)
      .for_each(|(lhs, rhs)| {
        *lhs -= rhs;
      });
    self.direct_map_2m_kib.iter_mut()
      .zip(rhs.direct_map_2m_kib)
      .for_each(|(lhs, rhs)| {
        *lhs -= rhs;
      });
    self.direct_map_4m_kib.iter_mut()
      .zip(rhs.direct_map_4m_kib)
      .for_each(|(lhs, rhs)| {
        *lhs -= rhs;
      });
    self.direct_map_1g_kib.iter_mut()
      .zip(rhs.direct_map_1g_kib)
      .for_each(|(lhs, rhs)| {
        *lhs -= rhs;
      });
    self.hardware_corrupted_kib.iter_mut()
      .zip(rhs.hardware_corrupted_kib)
      .for_each(|(lhs, rhs)| {
        *lhs -= rhs;
      });
    self.anon_huge_pages_kib.iter_mut()
      .zip(rhs.anon_huge_pages_kib)
      .for_each(|(lhs, rhs)| {
        *lhs -= rhs;
      });
    self.shmem_huge_pages_kib.iter_mut()
      .zip(rhs.shmem_huge_pages_kib)
      .for_each(|(lhs, rhs)| {
        *lhs -= rhs;
      });
    self.shmem_mapped_huge_pages_kib.iter_mut()
      .zip(rhs.shmem_mapped_huge_pages_kib)
      .for_each(|(lhs, rhs)| {
        *lhs -= rhs;
      });
    self.huge_pages_total_kib.iter_mut()
      .zip(rhs.huge_pages_total_kib)
      .for_each(|(lhs, rhs)| {
        *lhs -= rhs;
      });
    self.huge_pages_free_kib.iter_mut()
      .zip(rhs.huge_pages_free_kib)
      .for_each(|(lhs, rhs)| {
        *lhs -= rhs;
      });
    self.huge_pages_reserved_kib.iter_mut()
      .zip(rhs.huge_pages_reserved_kib)
      .for_each(|(lhs, rhs)| {
        *lhs -= rhs;
      });
    self.huge_pages_surplus_kib.iter_mut()
      .zip(rhs.huge_pages_surplus_kib)
      .for_each(|(lhs, rhs)| {
        *lhs -= rhs;
      });
    self.huge_page_size_kib.iter_mut()
      .zip(rhs.huge_page_size_kib)
      .for_each(|(lhs, rhs)| {
        *lhs -= rhs;
      });
  }
}

impl DivAssign<f64> for Memory {
  fn div_assign(&mut self, rhs: f64) {
    self.mem_total_kib /= rhs;
    self.mem_free_kib /= rhs;
    self.mem_available_kib /= rhs;
    self.cached_kib /= rhs;
    self.buffers_kib /= rhs;
    self.swap_cached_kib /= rhs;
    self.active_kib /= rhs;
    self.inactive_kib /= rhs;
    self.unevictable_kib /= rhs;
    self.mlocked_kib /= rhs;
    self.dirty_kib /= rhs;
    self.writeback_kib /= rhs;
    self.anon_pages_kib /= rhs;
    self.mapped_kib /= rhs;
    self.shmem_kib /= rhs;
    self.kreclaimable_kib /= rhs;
    self.slab_kib /= rhs;
    self.slab_reclaimable_kib /= rhs;
    self.slab_unreclaimable_kib /= rhs;
    self.kernel_stacks_kib /= rhs;
    self.page_tables_kib /= rhs;
    self.bounce_kib /= rhs;
    self.writeback_temp_kib /= rhs;
    
    self.lazy_free_kib.iter_mut()
      .for_each(|lhs| {
        *lhs /= rhs;
      });
    self.direct_map_4k_kib.iter_mut()
      .for_each(|lhs| {
        *lhs /= rhs;
      });
    self.direct_map_2m_kib.iter_mut()
      .for_each(|lhs| {
        *lhs /= rhs;
      });
    self.direct_map_4m_kib.iter_mut()
      .for_each(|lhs| {
        *lhs /= rhs;
      });
    self.direct_map_1g_kib.iter_mut()
      .for_each(|lhs| {
        *lhs /= rhs;
      });
    self.hardware_corrupted_kib.iter_mut()
      .for_each(|lhs| {
        *lhs /= rhs;
      });
    self.anon_huge_pages_kib.iter_mut()
      .for_each(|lhs| {
        *lhs /= rhs;
      });
    self.shmem_huge_pages_kib.iter_mut()
      .for_each(|lhs| {
        *lhs /= rhs;
      });
    self.shmem_mapped_huge_pages_kib.iter_mut()
      .for_each(|lhs| {
        *lhs /= rhs;
      });
    self.huge_pages_total_kib.iter_mut()
      .for_each(|lhs| {
        *lhs /= rhs;
      });
    self.huge_pages_free_kib.iter_mut()
      .for_each(|lhs| {
        *lhs /= rhs;
      });
    self.huge_pages_reserved_kib.iter_mut()
      .for_each(|lhs| {
        *lhs /= rhs;
      });
    self.huge_pages_surplus_kib.iter_mut()
      .for_each(|lhs| {
        *lhs /= rhs;
      });
    self.huge_page_size_kib.iter_mut()
      .for_each(|lhs| {
        *lhs /= rhs;
      });
  }
}

impl Sample<'_> for Memory {
  fn do_max_on_all_fields(&mut self, rhs: &Self) {
    self.mem_total_kib = f64::max(self.mem_total_kib, rhs.mem_total_kib);
    self.mem_free_kib = f64::max(self.mem_free_kib, rhs.mem_free_kib);
    self.mem_available_kib = f64::max(self.mem_available_kib, rhs.mem_available_kib);
    self.cached_kib = f64::max(self.cached_kib, rhs.cached_kib);
    self.buffers_kib = f64::max(self.buffers_kib, rhs.buffers_kib);
    self.swap_cached_kib = f64::max(self.swap_cached_kib, rhs.swap_cached_kib);
    self.active_kib = f64::max(self.active_kib, rhs.active_kib);
    self.inactive_kib = f64::max(self.inactive_kib, rhs.inactive_kib);
    self.unevictable_kib = f64::max(self.unevictable_kib, rhs.unevictable_kib);
    self.mlocked_kib = f64::max(self.mlocked_kib, rhs.mlocked_kib);
    self.dirty_kib = f64::max(self.dirty_kib, rhs.dirty_kib);
    self.writeback_kib = f64::max(self.writeback_kib, rhs.writeback_kib);
    self.anon_pages_kib = f64::max(self.anon_pages_kib, rhs.anon_pages_kib);
    self.mapped_kib = f64::max(self.mapped_kib, rhs.mapped_kib);
    self.shmem_kib = f64::max(self.shmem_kib, rhs.shmem_kib);
    self.kreclaimable_kib = f64::max(self.kreclaimable_kib, rhs.kreclaimable_kib);
    self.slab_kib = f64::max(self.slab_kib, rhs.slab_kib);
    self.slab_reclaimable_kib = f64::max(self.slab_reclaimable_kib, rhs.slab_reclaimable_kib);
    self.slab_unreclaimable_kib = f64::max(self.slab_unreclaimable_kib, rhs.slab_unreclaimable_kib);
    self.kernel_stacks_kib = f64::max(self.kernel_stacks_kib, rhs.kernel_stacks_kib);
    self.page_tables_kib = f64::max(self.page_tables_kib, rhs.page_tables_kib);
    self.bounce_kib = f64::max(self.bounce_kib, rhs.bounce_kib);
    self.writeback_temp_kib = f64::max(self.writeback_temp_kib, rhs.writeback_temp_kib);
    
    self.lazy_free_kib.iter_mut()
      .zip(rhs.lazy_free_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::max(*lhs, rhs);
      });
    self.direct_map_4k_kib.iter_mut()
      .zip(rhs.direct_map_4k_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::max(*lhs, rhs);
      });
    self.direct_map_2m_kib.iter_mut()
      .zip(rhs.direct_map_2m_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::max(*lhs, rhs);
      });
    self.direct_map_4m_kib.iter_mut()
      .zip(rhs.direct_map_4m_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::max(*lhs, rhs);
      });
    self.direct_map_1g_kib.iter_mut()
      .zip(rhs.direct_map_1g_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::max(*lhs, rhs);
      });
    self.hardware_corrupted_kib.iter_mut()
      .zip(rhs.hardware_corrupted_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::max(*lhs, rhs);
      });
    self.anon_huge_pages_kib.iter_mut()
      .zip(rhs.anon_huge_pages_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::max(*lhs, rhs);
      });
    self.shmem_huge_pages_kib.iter_mut()
      .zip(rhs.shmem_huge_pages_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::max(*lhs, rhs);
      });
    self.shmem_mapped_huge_pages_kib.iter_mut()
      .zip(rhs.shmem_mapped_huge_pages_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::max(*lhs, rhs);
      });
    self.huge_pages_total_kib.iter_mut()
      .zip(rhs.huge_pages_total_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::max(*lhs, rhs);
      });
    self.huge_pages_free_kib.iter_mut()
      .zip(rhs.huge_pages_free_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::max(*lhs, rhs);
      });
    self.huge_pages_reserved_kib.iter_mut()
      .zip(rhs.huge_pages_reserved_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::max(*lhs, rhs);
      });
    self.huge_pages_surplus_kib.iter_mut()
      .zip(rhs.huge_pages_surplus_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::max(*lhs, rhs);
      });
    self.huge_page_size_kib.iter_mut()
      .zip(rhs.huge_page_size_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::max(*lhs, rhs);
      });
  }
  
  fn do_min_on_all_fields(&mut self, rhs: &Self) {
    self.mem_total_kib = f64::min(self.mem_total_kib, rhs.mem_total_kib);
    self.mem_free_kib = f64::min(self.mem_free_kib, rhs.mem_free_kib);
    self.mem_available_kib = f64::min(self.mem_available_kib, rhs.mem_available_kib);
    self.cached_kib = f64::min(self.cached_kib, rhs.cached_kib);
    self.buffers_kib = f64::min(self.buffers_kib, rhs.buffers_kib);
    self.swap_cached_kib = f64::min(self.swap_cached_kib, rhs.swap_cached_kib);
    self.active_kib = f64::min(self.active_kib, rhs.active_kib);
    self.inactive_kib = f64::min(self.inactive_kib, rhs.inactive_kib);
    self.unevictable_kib = f64::min(self.unevictable_kib, rhs.unevictable_kib);
    self.mlocked_kib = f64::min(self.mlocked_kib, rhs.mlocked_kib);
    self.dirty_kib = f64::min(self.dirty_kib, rhs.dirty_kib);
    self.writeback_kib = f64::min(self.writeback_kib, rhs.writeback_kib);
    self.anon_pages_kib = f64::min(self.anon_pages_kib, rhs.anon_pages_kib);
    self.mapped_kib = f64::min(self.mapped_kib, rhs.mapped_kib);
    self.shmem_kib = f64::min(self.shmem_kib, rhs.shmem_kib);
    self.kreclaimable_kib = f64::min(self.kreclaimable_kib, rhs.kreclaimable_kib);
    self.slab_kib = f64::min(self.slab_kib, rhs.slab_kib);
    self.slab_reclaimable_kib = f64::min(self.slab_reclaimable_kib, rhs.slab_reclaimable_kib);
    self.slab_unreclaimable_kib = f64::min(self.slab_unreclaimable_kib, rhs.slab_unreclaimable_kib);
    self.kernel_stacks_kib = f64::min(self.kernel_stacks_kib, rhs.kernel_stacks_kib);
    self.page_tables_kib = f64::min(self.page_tables_kib, rhs.page_tables_kib);
    self.bounce_kib = f64::min(self.bounce_kib, rhs.bounce_kib);
    self.writeback_temp_kib = f64::min(self.writeback_temp_kib, rhs.writeback_temp_kib);
    
    self.lazy_free_kib.iter_mut()
      .zip(rhs.lazy_free_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::min(*lhs, rhs);
      });
    self.direct_map_4k_kib.iter_mut()
      .zip(rhs.direct_map_4k_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::min(*lhs, rhs);
      });
    self.direct_map_2m_kib.iter_mut()
      .zip(rhs.direct_map_2m_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::min(*lhs, rhs);
      });
    self.direct_map_4m_kib.iter_mut()
      .zip(rhs.direct_map_4m_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::min(*lhs, rhs);
      });
    self.direct_map_1g_kib.iter_mut()
      .zip(rhs.direct_map_1g_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::min(*lhs, rhs);
      });
    self.hardware_corrupted_kib.iter_mut()
      .zip(rhs.hardware_corrupted_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::min(*lhs, rhs);
      });
    self.anon_huge_pages_kib.iter_mut()
      .zip(rhs.anon_huge_pages_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::min(*lhs, rhs);
      });
    self.shmem_huge_pages_kib.iter_mut()
      .zip(rhs.shmem_huge_pages_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::min(*lhs, rhs);
      });
    self.shmem_mapped_huge_pages_kib.iter_mut()
      .zip(rhs.shmem_mapped_huge_pages_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::min(*lhs, rhs);
      });
    self.huge_pages_total_kib.iter_mut()
      .zip(rhs.huge_pages_total_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::min(*lhs, rhs);
      });
    self.huge_pages_free_kib.iter_mut()
      .zip(rhs.huge_pages_free_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::min(*lhs, rhs);
      });
    self.huge_pages_reserved_kib.iter_mut()
      .zip(rhs.huge_pages_reserved_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::min(*lhs, rhs);
      });
    self.huge_pages_surplus_kib.iter_mut()
      .zip(rhs.huge_pages_surplus_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::min(*lhs, rhs);
      });
    self.huge_page_size_kib.iter_mut()
      .zip(rhs.huge_page_size_kib)
      .for_each(|(lhs, rhs)| {
        *lhs = f64::min(*lhs, rhs);
      });
  }
}


