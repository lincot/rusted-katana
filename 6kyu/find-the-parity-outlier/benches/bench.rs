#![feature(test)]

extern crate test;
use find_the_parity_outlier::find_outlier;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut values = [5; if cfg!(miri) { 64 } else { 1024 }];
    values[values.len() / 2] = 4;
    bencher.iter(|| find_outlier(black_box(&values)));
}
