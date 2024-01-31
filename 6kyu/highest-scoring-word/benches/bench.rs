#![feature(test)]

extern crate test;
use highest_scoring_word::high;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| high(black_box("what time are we climbing up the volcano")));
}
