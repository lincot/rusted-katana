#![no_std]
#![feature(test)]

extern crate test;
use balanced_number_special_numbers_series_number_1::balanced_num;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(56_239_814);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(balanced_num(n));
        }
    });
}
