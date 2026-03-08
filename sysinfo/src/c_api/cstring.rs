use std::{slice, alloc::{Allocator, Global, Layout, handle_alloc_error}, ffi::{c_char, c_size_t}, ptr::NonNull, ascii};

// Like Rust's CString but its bare pointer + length
// the 'raw' also null terminated, C doesn't need to
// care about the "length" field
//
// But with additional guarantee such as this will only
// contains ASCII and has no NUL terminator so its cost
// free to cast to &str plus C won't write this and turn
// it to not ASCII safe
//
// Also UTF-8 is backward compatible with ASCII. which allows
// direct construction of &str in Rust
#[repr(C)]
pub struct CString {
  length: c_size_t,
  raw: NonNull<c_char>
}

static _UNUSED: () = assert!(size_of::<c_char>() == 1, "You're on exotic platform where C's char is not 8-bit");

impl Clone for CString {
  fn clone(&self) -> Self {
    todo!();
  }
}

impl Drop for CString {
  fn drop(&mut self) {
    let len = usize::try_from(c_size_t::try_from(self.length).unwrap() + 1).unwrap();
    unsafe { Global.deallocate(self.raw.cast(), Layout::new::<c_char>().repeat(len).unwrap().0); };
  }
}

impl CString {
  pub fn new(ascii_str: &[ascii::Char]) -> Option<Self> {
    if ascii_str.iter().find(|&&x| x == ascii::Char::Null).is_some() {
      return None;
    }
    
    let length = c_size_t::try_from(ascii_str.len()).unwrap();
    let bytes;
    let layout = Layout::new::<c_char>()
      .repeat(length + 1)
      .unwrap()
      .0;
    
    match Global.allocate(layout) {
      Ok(x) => bytes = x.cast(),
      Err(_) => handle_alloc_error(layout)
    }
    
    unsafe {  
      bytes.copy_from_nonoverlapping(NonNull::new_unchecked(ascii_str.as_ptr().cast_mut().cast()), ascii_str.len());
      
      // Place null terminator
      bytes.add(length).write(0);
    };
    
    Some(Self {
      length,
      raw: bytes
    })
  }
  
  pub fn as_str(&self) -> &str {
    self.as_ref()
  }
}

pub enum ConvertToASCIICStringError {
  ContainsNull,
  ContainsNonAscii
}

impl TryFrom<String> for CString {
  type Error = ConvertToASCIICStringError;
  
  fn try_from(value: String) -> Result<Self, Self::Error> {
    Self::try_from(value.as_str())
  }
}

impl TryFrom<&str> for CString {
  type Error = ConvertToASCIICStringError;
  
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let ascii = value.as_ascii()
      .ok_or(ConvertToASCIICStringError::ContainsNonAscii)?;
    
    Self::new(ascii)
      .ok_or(ConvertToASCIICStringError::ContainsNull)
  }
}

impl AsRef<str> for CString {
  fn as_ref(&self) -> &str {
    // SAFETY: length is known and C won't write this and its always contains
    // ASCII and also valid UTF-8 :3
    unsafe {
      str::from_utf8_unchecked(slice::from_raw_parts(self.raw.as_ptr().cast(), usize::try_from(self.length).unwrap()))
    }
  }
}

