#![feature(test)]

extern crate test;
use string_letter_counting::string_letter_count;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("The quick brown fox jumps over the lazy dog.");
    bencher.iter(|| string_letter_count(s));
}
