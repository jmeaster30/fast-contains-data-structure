extern crate rand;

use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::{distributions::Alphanumeric, Rng};
use test::Bencher;
use std::iter;
use std::str;

use super::BloomFilter;


#[test]
fn test_bloom_filter_add() {
  let mut bloom_filter: BloomFilter<500, 10> = BloomFilter::new();

  bloom_filter.add("test".to_string());

  let bits_set = bloom_filter.bit_vec.count_ones();
  assert!(1 <= bits_set && bits_set <= 10);
}

#[test]
fn test_bloom_filter_test_positive() {
  let mut bloom_filter: BloomFilter<500, 10> = BloomFilter::new();

  let to_add = "test";
  let to_test = "test";

  bloom_filter.add(to_add.to_string());

  assert!(bloom_filter.test(to_test.to_string()));
}

#[test]
fn test_bloom_filter_test_negative() {
  let mut bloom_filter: BloomFilter<500, 10> = BloomFilter::new();

  let to_add = "test";
  let to_test = "lily";

  bloom_filter.add(to_add.to_string());

  assert!(!bloom_filter.test(to_test.to_string()));
}

fn generate_strings(rng: &mut ThreadRng, length: usize, count: usize) -> Vec<String> {
  iter::repeat(())
    .map(|()| rng.sample(Alphanumeric))
    .take(length * count)
    .collect::<Vec<u8>>()
    .chunks(length)
    .map(|buf| str::from_utf8(buf).unwrap().to_string())
    .collect::<Vec<String>>()
}

#[bench]
fn bench_bloom_filter_add(b: &mut Bencher) {
  let strings = generate_strings(&mut rand::thread_rng(), 10, 500);
  
  b.iter(|| {
    let mut bloom_filter: BloomFilter<23963, 17> = BloomFilter::new();
    for s in &strings {
      bloom_filter.add(s.clone());
    }
  });
}

#[bench]
fn bench_bloom_filter_test(b: &mut Bencher) {
  let mut rng = rand::thread_rng();

  let strings = generate_strings(&mut rng, 10, 500);
  let mut bloom_filter: BloomFilter<23963, 17> = BloomFilter::new();
  for s in &strings {
    bloom_filter.add(s.clone());
  }
  
  b.iter(|| {
    for s in strings.choose_multiple(&mut rng, 500) {
      bloom_filter.test(s.clone());
    }
  });
}

#[bench]
fn bench_bloom_filter_test_with_fails(b: &mut Bencher) {
  let mut rng = rand::thread_rng();

  let strings = generate_strings(&mut rng, 10, 500);
  
  let mut bloom_filter: BloomFilter<23963, 17> = BloomFilter::new();
  for s in strings {
    bloom_filter.add(s);
  }

  let to_search = generate_strings(&mut rng, 10, 500);
  
  b.iter(|| {
    for s in to_search.choose_multiple(&mut rng, 500) {
      bloom_filter.test(s.clone());
    }
  });
}