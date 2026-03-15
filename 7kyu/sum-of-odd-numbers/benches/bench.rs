#![feature(test)]

extern crate test;
use sum_of_odd_numbers::row_sum_odd_numbers;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| row_sum_odd_numbers(black_box(42)));
}
