#![feature(test)]

extern crate test;
use jumping_number_special_numbers_series_number_4::jumping_number;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(jumping_number(black_box(98_789_876)));
        }
    });
}
