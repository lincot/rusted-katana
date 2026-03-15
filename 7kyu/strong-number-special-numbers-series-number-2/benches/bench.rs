#![feature(test)]

extern crate test;
use strong_number_special_numbers_series_number_2::strong;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| strong(black_box(40_000)));
}
