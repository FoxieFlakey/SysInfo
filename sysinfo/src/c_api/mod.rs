use std::{ffi::c_void, ptr::NonNull};

use crate::{SysInfo, capturers::{cpu::CPU, memory::Memory, swap::Swaps}, metric::{Capturer, Metric}};

pub mod option;
pub mod cvec;
pub mod cstring;

#[unsafe(no_mangle)]
pub extern "C" fn sysinfo_new() -> *mut c_void {
  Box::into_raw(Box::new(SysInfo::new())) as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sysinfo_free(this: *mut c_void) {
  // That is C's problem
  unsafe { drop(Box::from_raw(this)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sysinfo_update(this: *mut c_void) {
  let instance = unsafe { this.cast::<SysInfo>().as_mut().expect("C gave null pointer") };
  instance.update();
}

fn get_xxx_sample_impl<T: Capturer>(metric: &Metric<T>) -> Option<NonNull<T::Sample>> {
  metric.data
    .samples
    .front()?
    .as_ref()
    .map(NonNull::from_ref)
    .map(NonNull::cast)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sysinfo_get_latest_memory_sample(this: *const c_void) -> Option<NonNull<Memory>> {
  let instance = unsafe { this.cast::<SysInfo>().as_ref().expect("C gave null pointer") };
  get_xxx_sample_impl(&instance.memory_usage)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sysinfo_get_latest_swap_sample(this: *const c_void) -> Option<NonNull<Swaps>> {
  let instance = unsafe { this.cast::<SysInfo>().as_ref().expect("C gave null pointer") };
  get_xxx_sample_impl(&instance.swap_usage)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sysinfo_get_latest_cpu_sample(this: *const c_void) -> Option<NonNull<CPU>> {
  let instance = unsafe { this.cast::<SysInfo>().as_ref().expect("C gave null pointer") };
  get_xxx_sample_impl(&instance.cpu_usage)
}

