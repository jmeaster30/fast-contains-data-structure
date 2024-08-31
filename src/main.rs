mod bloom_filter;

use rand::prelude::SliceRandom;

use bloom_filter::BloomFilter;

fn main() {
  return;
  let mut bloom_filter: BloomFilter<60, 4> = BloomFilter::new();

  let inserted_strings = vec!["google.com", "youtube.com", "syrency.com", "spotify.com", "mycoolsite.gov", "cybi.lily", "github.com", "comcast.net"];
  let not_inserted_strings = vec!["reddit.com", "rust.org", "facebook.com", "insta.gram", "linkedin.club", "agar.io"];

  bloom_filter.add_string_refs(&inserted_strings);

  let mut total_correct_inserted = 0;
  let mut total_wrong_inserted = 0;
  let mut total_correct_not_inserted = 0;
  let mut total_wrong_not_inserted = 0;
  for _i in 0..50 {
    let find_inserted = inserted_strings.choose(&mut rand::thread_rng()).unwrap().to_string();
    let find_not_inserted = not_inserted_strings.choose(&mut rand::thread_rng()).unwrap().to_string();
    let found_inserted = bloom_filter.test(find_inserted.clone());
    let found_not_inserted = bloom_filter.test(find_not_inserted.clone());
    println!("Contains '{}'? {}", find_inserted, if found_inserted { "TRUE" } else { "FALSE" });
    println!("Contains '{}'? {}", find_not_inserted, if found_not_inserted { "TRUE" } else { "FALSE" });

    if found_inserted {
      total_correct_inserted += 1;  
    } else {
      total_wrong_inserted += 1;
    }

    if !found_not_inserted {
      total_correct_not_inserted += 1;
    } else {
      total_wrong_not_inserted += 1;
    }
  }

  println!("Total Correct Inserted: {}", total_correct_inserted);
  println!("Total Wrong Inserted: {}", total_wrong_inserted);
  println!("Total Correct Not Inserted: {}", total_correct_not_inserted);
  println!("Total Wrong Not Inserted: {}", total_wrong_not_inserted);


}