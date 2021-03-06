#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use tidy_number_special_numbers_series_number_9::tidy_number;

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(5_123_456_789);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(tidy_number(n));
        }
    });
}
