#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("The quick brown fox jumps over the lazy dog.");
    bencher.iter(|| solution::string_letter_count(s));
}
