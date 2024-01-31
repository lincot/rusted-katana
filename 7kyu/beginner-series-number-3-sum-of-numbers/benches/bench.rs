#![feature(test)]

extern crate test;
use beginner_series_number_3_sum_of_numbers::get_sum;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(get_sum(black_box(10_000), black_box(-10_000)));
        }
    });
}
