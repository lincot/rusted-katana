#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use tidy_number_special_numbers_series_number_9::tidy_number;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(tidy_number(black_box(5_123_456_789)));
        }
    });
}
