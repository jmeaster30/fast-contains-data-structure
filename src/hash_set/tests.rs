extern crate rand;

use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::{distributions::Alphanumeric, Rng};
use test::Bencher;
use std::iter;
use std::str;

use super::HashSet;


#[test]
fn test_hash_set_add() {
  let mut hash_set: HashSet<500> = HashSet::new();

  hash_set.add("test".to_string());

  assert_eq!(1, hash_set.contents.len());
}

#[test]
fn test_hash_set_test_positive() {
  let mut hash_set: HashSet<500> = HashSet::new();

  let to_add = "test";
  let to_test = "test";

  hash_set.add(to_add.to_string());

  assert!(hash_set.test(to_test.to_string()));
}

#[test]
fn test_hash_set_test_negative() {
  let mut hash_set: HashSet<500> = HashSet::new();

  let to_add = "test";
  let to_test = "lily";

  hash_set.add(to_add.to_string());

  assert!(!hash_set.test(to_test.to_string()));
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
fn bench_hash_set_add(b: &mut Bencher) {
  let strings = generate_strings(&mut rand::thread_rng(), 10, 500);
  
  b.iter(|| {
    let mut hash_set: HashSet<500> = HashSet::new();
    for s in &strings {
      hash_set.add(s.clone());
    }
  });
}

#[bench]
fn bench_hash_set_test(b: &mut Bencher) {
  let mut rng = rand::thread_rng();

  let strings = generate_strings(&mut rng, 10, 500);
  let mut hash_set: HashSet<500> = HashSet::new();
  for s in &strings {
    hash_set.add(s.clone());
  }
  
  b.iter(|| {
    for s in strings.choose_multiple(&mut rng, 500) {
      hash_set.test(s.clone());
    }
  });
}

#[bench]
fn bench_hash_set_test_with_fails(b: &mut Bencher) {
  let mut rng = rand::thread_rng();

  let strings = generate_strings(&mut rng, 10, 500);
  
  let mut hash_set: HashSet<500> = HashSet::new();
  for s in strings {
    hash_set.add(s);
  }

  let to_search = generate_strings(&mut rng, 10, 500);
  
  b.iter(|| {
    for s in to_search.choose_multiple(&mut rng, 500) {
      hash_set.test(s.clone());
    }
  });
}