#[cfg(test)]
mod tests;

use std::hash::{DefaultHasher, Hash, Hasher};

use bitvec::prelude::*;

pub struct BloomFilter<const BIT_ARRAY_LENGTH: usize, const HASH_FUNCTION_COUNT: usize> {
  bit_vec: BitArray<[usize; BIT_ARRAY_LENGTH], Lsb0>
}

impl<const BIT_ARRAY_LENGTH: usize, const HASH_FUNCTION_COUNT: usize> BloomFilter<BIT_ARRAY_LENGTH, HASH_FUNCTION_COUNT> {
  pub fn new() -> Self {
    Self { 
      bit_vec: BitArray::ZERO,
    }
  }

  fn get_hashes(value: String) -> [usize; HASH_FUNCTION_COUNT] {
    let mut hash_function = DefaultHasher::new();
    value.hash(&mut hash_function);
    let mut current = hash_function.finish() as usize;
    let mut hashes = [current % BIT_ARRAY_LENGTH; HASH_FUNCTION_COUNT];
    for i in 1..HASH_FUNCTION_COUNT {
      current.hash(&mut hash_function);
      current = hash_function.finish() as usize;
      hashes[i] = current % BIT_ARRAY_LENGTH;
    }
    return hashes;
  }

  pub fn add(&mut self, value: String) {
    let hashes = Self::get_hashes(value);

    for hash in hashes {
      self.bit_vec.set(hash, true);
    }
  }

  pub fn add_strings(&mut self, values: Vec<String>) {
    for value in values {
      self.add(value)
    }
  }

  pub fn add_string_refs(&mut self, values: &Vec<&str>) {
    for value in values {
      self.add(value.to_string())
    }
  }

  pub fn test(&self, value: String) -> bool {
    let hashes = Self::get_hashes(value);

    // if any are 0 then we are NOT in the set
    // if all are 1 then it is probably in the set
    return hashes.iter()
      .map(|idx| self.bit_vec.get(*idx))
      .all(|bit| bit.is_some() && *bit.unwrap())
  }
}

