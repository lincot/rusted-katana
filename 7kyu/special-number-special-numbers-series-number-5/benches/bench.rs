#![feature(test)]

extern crate test;
use special_number_special_numbers_series_number_5::special_number;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| special_number(black_box(123_450_123_450_123_450)));
}
