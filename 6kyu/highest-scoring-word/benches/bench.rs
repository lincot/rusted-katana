#![no_std]
#![feature(test)]

extern crate test;
use highest_scoring_word::high;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let input = black_box("what time are we climbing up the volcano");
    bencher.iter(|| high(input));
}
