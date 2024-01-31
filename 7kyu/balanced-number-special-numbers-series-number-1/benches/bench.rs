#![feature(test)]

extern crate test;
use balanced_number_special_numbers_series_number_1::balanced_num;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(balanced_num(black_box(56_239_814)));
        }
    });
}
