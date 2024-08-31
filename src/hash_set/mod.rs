#[cfg(test)]
mod tests;

use std::hash::{DefaultHasher, Hash, Hasher};

pub struct HashSet<const CAPACITY: usize> {
  contents: [u64; CAPACITY],
  size: usize
}

impl<const CAPACITY: usize> HashSet<CAPACITY> {
  pub fn new() -> Self {
    Self { 
      contents: [0; CAPACITY],
      size: 0,
    }
  }

  pub fn size(&self) -> usize {
    self.size
  } 

  pub fn add(&mut self, value: String) {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    self.add_if_not_exists(hasher.finish());
    self.contents.sort();
  }

  fn add_if_not_exists(&mut self, value: u64) {
    if self.contents.binary_search(&value).is_ok() {
      ()
    } else {
      self.contents[self.size] = value;
      self.size += 1;
    }
  }

  pub fn test(&self, value: String) -> bool {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    self.contents.binary_search(&hasher.finish()).is_ok()
  }
}