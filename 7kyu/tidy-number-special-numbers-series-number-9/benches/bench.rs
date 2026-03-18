#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use tidy_number_special_numbers_series_number_9::tidy_number;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| tidy_number(black_box(5_123_456_789)));
}
