use std::{fs::File, io::{BufRead, BufReader}};

use crate::{capturers::memory::Memory, metric::Capturer};

pub struct MemoryCapturer;

impl MemoryCapturer {
  pub fn new() -> Self {
    Self
  }
}

impl Capturer for MemoryCapturer {
  type Sample = Memory;
  
  fn capture(&mut self) -> Option<Self::Sample> {
    let lines = BufReader::new(File::open("/proc/meminfo").ok()?).lines();
    
    let mut mem_total_kib = None;
    let mut mem_free_kib = None;
    let mut mem_available_kib = None;
    let mut cached_kib = None;
    let mut buffers_kib = None;
    let mut swap_cached_kib = None;
    let mut active_kib = None;
    let mut inactive_kib = None;
    let mut unevictable_kib = None;
    let mut mlocked_kib = None;
    let mut dirty_kib = None;
    let mut writeback_kib = None;
    let mut anon_pages_kib = None;
    let mut mapped_kib = None;
    let mut kreclaimable_kib = None;
    let mut shmem_kib = None;
    let mut slab_kib = None;
    let mut slab_reclaimable_kib = None;
    let mut slab_unreclaimable_kib = None;
    let mut kernel_stacks_kib = None;
    let mut page_tables_kib = None;
    let mut bounce_kib = None;
    let mut writeback_temp_kib = None;
    let mut lazy_free_kib = None;
    let mut direct_map_4k_kib = None;
    let mut direct_map_2m_kib = None;
    let mut direct_map_4m_kib = None;
    let mut direct_map_1g_kib = None;
    let mut hardware_corrupted_kib = None;
    let mut anon_huge_pages_kib = None;
    let mut shmem_huge_pages_kib = None;
    let mut shmem_mapped_huge_pages_kib = None;
    let mut huge_pages_total_kib = None;
    let mut huge_pages_free_kib = None;
    let mut huge_pages_reserved_kib = None;
    let mut huge_pages_surplus_kib = None;
    let mut huge_page_size_kib = None;
    
    for line in lines {
      let line = line.ok()?;
      let mut fields = line.split_whitespace();
      let name = fields.next()?;
      let size_kib = fields.next()?;
      
      // Stuffs that starts without HugePages required unit
      if !name.starts_with("HugePages_") {
        let unit = fields.next()?;
        
        if unit != "kB" {
          return None;
        }
      }
      
      if size_kib.starts_with('-') {
        return None;
      }
      
      let size_kib = u64::from_str_radix(size_kib, 10).ok()? as f64;
      
      match name {
        "MemTotal:" => mem_total_kib = Some(size_kib),
        "MemFree:" => mem_free_kib = Some(size_kib),
        "MemAvailable:" => mem_available_kib = Some(size_kib),
        "Buffers:" => buffers_kib = Some(size_kib),
        "Cached:" => cached_kib = Some(size_kib),
        "SwapCached:" => swap_cached_kib = Some(size_kib),
        "Active:" => active_kib = Some(size_kib),
        "Inactive:" => inactive_kib = Some(size_kib),
        "Unevictable:" => unevictable_kib = Some(size_kib),
        "Mlocked:" => mlocked_kib = Some(size_kib),
        "Dirty:" => dirty_kib = Some(size_kib),
        "Writeback:" => writeback_kib = Some(size_kib),
        "AnonPages:" => anon_pages_kib = Some(size_kib),
        "Mapped:" => mapped_kib = Some(size_kib),
        "Shmem:" => shmem_kib = Some(size_kib),
        "KReclaimable:" => kreclaimable_kib = Some(size_kib),
        "Slab:" => slab_kib = Some(size_kib),
        "SReclaimable:" => slab_reclaimable_kib = Some(size_kib),
        "SUnreclaim:" => slab_unreclaimable_kib = Some(size_kib),
        "KernelStack:" => kernel_stacks_kib = Some(size_kib),
        "PageTables:" => page_tables_kib = Some(size_kib),
        "Bounce:" => bounce_kib = Some(size_kib),
        "WritebackTmp:" => writeback_temp_kib = Some(size_kib),
        "LazyFree:" => lazy_free_kib = Some(size_kib),
        "DirectMap4k:" => direct_map_4k_kib = Some(size_kib),
        "DirectMap2M:" => direct_map_2m_kib = Some(size_kib),
        "DirectMap4M:" => direct_map_4m_kib = Some(size_kib),
        "DirectMap1G:" => direct_map_1g_kib = Some(size_kib),
        "HardwareCorrupted:" => hardware_corrupted_kib = Some(size_kib),
        "AnonHugePages:" => anon_huge_pages_kib = Some(size_kib),
        "ShmemHugePages:" => shmem_huge_pages_kib = Some(size_kib),
        "ShmemPmdMapped:" => shmem_mapped_huge_pages_kib = Some(size_kib),
        "HugePages_Total:" => huge_pages_total_kib = Some(size_kib),
        "HugePages_Free:" => huge_pages_free_kib = Some(size_kib),
        "HugePages_Rsvd:" => huge_pages_reserved_kib = Some(size_kib),
        "HugePages_Surp:" => huge_pages_surplus_kib = Some(size_kib),
        "Hugepagesize:" => huge_page_size_kib = Some(size_kib),
        
        // Ignore the field
        _ => ()
      }
    }
    
    // NOTE: The huge_pages_* still contains number of pages NOT in KiB
    if let Some(size) = huge_page_size_kib {
      huge_pages_total_kib = huge_pages_total_kib.map(|x| x * size);
      huge_pages_free_kib = huge_pages_free_kib.map(|x| x * size);
      huge_pages_reserved_kib = huge_pages_reserved_kib.map(|x| x * size);
      huge_pages_surplus_kib = huge_pages_surplus_kib.map(|x| x * size);
    } else {
      huge_pages_total_kib = None;
      huge_pages_free_kib = None;
      huge_pages_reserved_kib = None;
      huge_pages_surplus_kib = None;
    }
    
    Some(Memory {
      mem_total_kib: mem_total_kib?,
      mem_free_kib: mem_free_kib?,
      mem_available_kib: mem_available_kib?,
      cached_kib: cached_kib?,
      buffers_kib: buffers_kib?,
      swap_cached_kib: swap_cached_kib?,
      active_kib: active_kib?,
      inactive_kib: inactive_kib?,
      unevictable_kib: unevictable_kib?,
      mlocked_kib: mlocked_kib?,
      dirty_kib: dirty_kib?,
      writeback_kib: writeback_kib?,
      anon_pages_kib: anon_pages_kib?,
      mapped_kib: mapped_kib?,
      kreclaimable_kib: kreclaimable_kib?,
      shmem_kib: shmem_kib?,
      slab_kib: slab_kib?,
      slab_reclaimable_kib: slab_reclaimable_kib?,
      slab_unreclaimable_kib: slab_unreclaimable_kib?,
      kernel_stacks_kib: kernel_stacks_kib?,
      page_tables_kib: page_tables_kib?,
      bounce_kib: bounce_kib?,
      writeback_temp_kib: writeback_temp_kib?,
      
      lazy_free_kib,
      direct_map_4k_kib,
      direct_map_2m_kib,
      direct_map_4m_kib,
      direct_map_1g_kib,
      hardware_corrupted_kib,
      anon_huge_pages_kib,
      shmem_huge_pages_kib,
      shmem_mapped_huge_pages_kib,
      huge_pages_total_kib,
      huge_pages_free_kib,
      huge_pages_reserved_kib,
      huge_pages_surplus_kib,
      huge_page_size_kib
    })
  }
}

