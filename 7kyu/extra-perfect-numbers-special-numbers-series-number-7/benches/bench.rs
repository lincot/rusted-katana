#![feature(test)]

extern crate test;
use extra_perfect_numbers_special_numbers_series_number_7::extra_perfect;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(1000);
    bencher.iter(|| extra_perfect(n));
}
