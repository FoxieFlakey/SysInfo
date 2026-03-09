use std::{alloc::{Allocator, Global, Layout, handle_alloc_error}, ffi::c_size_t, ptr::NonNull};

#[repr(C)]
pub struct CVec<T: Unpin> {
  length: c_size_t,
  capacity: c_size_t,
  data: Option<NonNull<T>>
}

impl<T: Unpin> Drop for CVec<T> {
  fn drop(&mut self) {
    if let Some(data) = self.data.take() {
      for i in 0..self.length {
        let i = usize::try_from(i).unwrap();
        
        // SAFETY: It is in current allocation
        unsafe { data.add(i).drop_in_place() };
      }
      
      // Dealloc it
      unsafe { Global.deallocate(data.cast(), Layout::new::<T>().repeat(self.capacity).unwrap().0) };
    }
  }
}

impl<T: Unpin> CVec<T> {
  pub fn new() -> Self {
    Self {
      length: 0,
      capacity: 0,
      data: None
    }
  }
  
  pub fn push(&mut self, v: T) {
    self.ensure_capacity(self.length + 1);
    
    // Already ensure there space by previous call
    unsafe {
      // add to the list
      self.data.unwrap()
        .add(usize::try_from(self.length).unwrap())
        .write(v);
    }
    
    self.length += 1;
  }
  
  fn ensure_capacity(&mut self, expected: c_size_t) {
    let expected = usize::try_from(expected).unwrap();
    
    if self.capacity >= expected {
      return;
    }
    
    let new_capacity = if expected.is_power_of_two() { expected } else { expected.next_power_of_two() };
    let old_layout = Layout::new::<T>()
      .repeat(usize::try_from(self.capacity).unwrap())
      .unwrap()
      .0;
    let layout = Layout::new::<T>()
      .repeat(new_capacity)
      .unwrap()
      .0;
    
    let res = match self.data {
      // SAFETY: The data is safe to be moved, and its certain growing due checks earlier
      // on new_capacity <= self.capacity
      Some(existing) => unsafe { Global.grow(existing.cast(), old_layout, layout) }
      None => Global.allocate(layout)
    };
    
    match res {
      Ok(x) => {
        self.data = Some(x.cast());
        self.capacity = c_size_t::try_from(new_capacity).unwrap();
      }
      
      Err(_) => handle_alloc_error(layout)
    }
  }
  
  pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    Iter {
      owner: self,
      current: 0
    }
  }
  
  pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
    IterMut {
      owner: self,
      current: 0
    }
  }
  
  pub fn len(&self) -> usize {
    usize::try_from(self.length).unwrap()
  }
}

impl<T: Clone + Unpin> Clone for CVec<T> {
  fn clone(&self) -> Self {
    let mut cloned = Self::new();
    cloned.ensure_capacity(self.length);
    
    if let Some(src) = self.data {
      let dest = cloned.data.unwrap();
      for i in 0..self.length {
        // SAFETY: Already know the bound of the space by self.length
        unsafe {
          dest.add(i)
            .write(src.add(i)
            .as_ref()
            .clone()
          )
        };
      }
    }
    
    cloned.length = self.length;
    cloned
  }
}

pub struct Iter<'a, T: Unpin> {
  owner: &'a CVec<T>,
  current: c_size_t
}

impl<'a, T: Unpin> Iterator for Iter<'a, T> {
  type Item = &'a T;
  
  fn next(&mut self) -> Option<Self::Item> {
    if self.current < self.owner.length {
      let ptr = self.owner.data.expect("The length is non zero but data is None (that mean nothing allocated");
      let ret = unsafe { ptr.add(usize::try_from(self.current).unwrap()).as_ref() };
      self.current += 1;
      
      Some(ret)
    } else {
      None
    }
  }
}

pub struct IterMut<'a, T: Unpin> {
  owner: &'a mut CVec<T>,
  current: c_size_t
}

impl<'a, T: Unpin> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;
  
  fn next(&mut self) -> Option<Self::Item> {
    if self.current < self.owner.length {
      let ptr = self.owner.data.expect("The length is non zero but data is None (that mean nothing allocated");
      let ret = unsafe { ptr.add(usize::try_from(self.current).unwrap()).as_mut() };
      self.current += 1;
      
      Some(ret)
    } else {
      None
    }
  }
}

impl<'a, T: Unpin> IntoIterator for &'a CVec<T> {
  type Item = &'a T;
  type IntoIter = Iter<'a, T>;
  
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

