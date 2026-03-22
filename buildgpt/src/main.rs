mod tokenizer;
use std::env;
use tokenizer::tokenize_test;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("usage: cargo run -- <path-to-file>");
    tokenize_test(&path);
    return;
}
