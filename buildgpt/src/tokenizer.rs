// things we need:
// 1. A way to load a corpus
// 2. A way to process the corpus
// 3. The actual tokenizer logic
// 4. A way to write the tokenized data to storage
//
// let's start with tokenizing a simple example text.

use std::{collections::HashMap, fs};

pub fn print_bytes(b: &[u8]) -> () {
    println!(
        "{}",
        b.iter()
            .map(|byte| format!("{byte:08b}"))
            .collect::<Vec<_>>()
            .join(" ")
    );
}

pub fn count_byte_pairs(b: &[u8]) -> HashMap<(u8, u8), usize> {
    let mut pair_counts: HashMap<(u8, u8), usize> = HashMap::new();
    for window in b.windows(2) {
        let key = (window[0], window[1]);
        *pair_counts.entry(key).or_insert(0) += 1;
    }
    pair_counts
}

pub fn most_counted_pair(map: &HashMap<(u8, u8), usize>) -> (u8, u8) {
    map.iter()
        .max_by_key(|(_, count)| *count)
        .map(|(pair, _)| *pair)
        .unwrap()
}

pub fn tokenize_test(path: &str) -> () {
    // load the first line of the txt file
    let contents = fs::read_to_string(path).expect("faile to read file");
    let s = &contents[0..512];
    println!("{}", s);
    let b = s.as_bytes();
    let pair_counts = count_byte_pairs(&b);
    let pair = most_counted_pair(&pair_counts);
    let a = pair.0;
    let b = pair.1;
    println!("most counted bytes: ({a}, {b})");
    println!("those are: ({}, {})", a as char, b as char);
    //print_bytes(b);
}
