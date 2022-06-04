#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench_positive(bencher: &mut Bencher) {
    let n = black_box(1000);
    bencher.iter(|| solution::sum_of_n(n))
}

#[bench]
fn bench_negative(bencher: &mut Bencher) {
    let n = black_box(-1000);
    bencher.iter(|| solution::sum_of_n(n))
}
