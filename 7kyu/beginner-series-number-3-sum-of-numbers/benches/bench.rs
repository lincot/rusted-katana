#![no_std]
#![feature(test)]

extern crate test;
use beginner_series_number_3_sum_of_numbers::get_sum;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box(10_000);
    let b = black_box(-10_000);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(get_sum(a, b));
        }
    });
}
