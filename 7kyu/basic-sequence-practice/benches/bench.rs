#![no_std]
#![feature(test)]

extern crate test;
use basic_sequence_practice::sum_of_n;
use test::{black_box, Bencher};

#[bench]
fn bench_positive(bencher: &mut Bencher) {
    let n = black_box(1000);
    bencher.iter(|| sum_of_n(n));
}

#[bench]
fn bench_negative(bencher: &mut Bencher) {
    let n = black_box(-1000);
    bencher.iter(|| sum_of_n(n));
}
