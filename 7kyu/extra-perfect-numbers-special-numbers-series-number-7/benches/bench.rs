#![no_std]
#![feature(test)]

extern crate test;
use extra_perfect_numbers_special_numbers_series_number_7::extra_perfect;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| extra_perfect(black_box(1000)));
}
