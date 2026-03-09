// Its a wrapper around Rust's Option so C can use it

use std::{mem::MaybeUninit, option};

#[repr(C)]
pub struct COption<T> {
  is_present: bool,
  data: MaybeUninit<T>
}

impl<T> Drop for COption<T> {
  fn drop(&mut self) {
    if self.is_present {
      unsafe { self.data.assume_init_drop(); };
    }
  }
}

impl<T> Into<Option<T>> for COption<T> {
  fn into(self) -> Option<T> {
    self.into_opt()
  }
}

impl<T> From<Option<T>> for COption<T> {
  fn from(value: Option<T>) -> Self {
    match value {
      Some(x) => Self {
        is_present: true,
        data: MaybeUninit::new(x)
      },
      
      None => Self {
        is_present: false,
        data: MaybeUninit::uninit()
      }
    }
  }
}

impl<T: Clone> Clone for COption<T> {
  fn clone(&self) -> Self {
    let cloned;
    
    if self.is_present {
      cloned = MaybeUninit::new(unsafe { self.data.assume_init_ref() }.clone());
    } else {
      cloned = MaybeUninit::uninit();
    }
    
    Self {
      is_present: self.is_present,
      data: cloned
    }
  }
}

impl<T> COption<T> {
  pub fn as_ref<'a>(&'a self) -> COption<&'a T> {
    let reference;
    
    if self.is_present {
      reference = MaybeUninit::new(unsafe { self.data.assume_init_ref() });
    } else {
      reference = MaybeUninit::uninit();
    }
    
    COption {
      is_present: self.is_present,
      data: reference
    }
  }
  
  pub fn as_mut<'a>(&'a mut self) -> COption<&'a mut T> {
    let reference;
    
    if self.is_present {
      reference = MaybeUninit::new(unsafe { self.data.assume_init_mut() });
    } else {
      reference = MaybeUninit::uninit();
    }
    
    COption {
      is_present: self.is_present,
      data: reference
    }
  }
  
  pub fn into_opt(mut self) -> Option<T> {
    if self.is_present {
      // Ensure drop doesn't trigger
      // we already moved out data in there
      self.is_present = false;
      Some(unsafe { self.data.assume_init_read() })
    } else {
      None
    }
  }
  
  pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
    IterMut { option: self.as_mut().into_opt().into_iter() }
  }
}

impl<'a, T> IntoIterator for &'a COption<T> {
  type IntoIter = IntoIter<&'a T>;
  type Item = &'a T;
  
  fn into_iter(self) -> Self::IntoIter {
    IntoIter {
      option: self.as_ref().into_opt().into_iter()
    }
  }
}

impl<T> IntoIterator for COption<T> {
  type IntoIter = IntoIter<T>;
  type Item = T;
  
  fn into_iter(self) -> Self::IntoIter {
    IntoIter {
      option: self.into_opt().into_iter()
    }
  }
}

pub struct IterMut<'a, T> {
  option: option::IntoIter<&'a mut T>
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;
  
  fn next(&mut self) -> Option<Self::Item> {
    self.option.next()
  }
}

pub struct IntoIter<T> {
  option: option::IntoIter<T>
}

impl<T> Iterator for IntoIter<T> {
  type Item = T;
  
  fn next(&mut self) -> Option<Self::Item> {
    self.option.next()
  }
}


