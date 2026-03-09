use std::{ffi::c_void, ptr::NonNull};

use crate::SysInfo;

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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sysinfo_get_latest_memory_sample(this: *const c_void) -> Option<NonNull<c_void>> {
  let instance = unsafe { this.cast::<SysInfo>().as_ref().expect("C gave null pointer") };
  instance.memory_usage
    .data
    .samples
    .front()?
    .as_ref()
    .map(NonNull::from_ref)
    .map(NonNull::cast)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sysinfo_get_latest_swap_sample(this: *const c_void) -> Option<NonNull<c_void>> {
  let instance = unsafe { this.cast::<SysInfo>().as_ref().expect("C gave null pointer") };
  instance.swap_usage
    .data
    .samples
    .front()?
    .as_ref()
    .map(NonNull::from_ref)
    .map(NonNull::cast)
}

