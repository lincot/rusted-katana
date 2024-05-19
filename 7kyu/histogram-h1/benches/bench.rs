#![feature(test)]

extern crate test;
use histogram_h1::histogram;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| histogram(black_box(&[7, 3, 10, 1, 0, 5])));
}
