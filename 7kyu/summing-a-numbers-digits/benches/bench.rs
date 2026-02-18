#![feature(test)]

extern crate test;
use summing_a_numbers_digits::sum_digits;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| sum_digits(black_box(-1_234_567_890)));
}
