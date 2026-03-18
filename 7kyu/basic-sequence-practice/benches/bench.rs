#![feature(test)]

extern crate test;
use basic_sequence_practice::sum_of_n;
use test::{Bencher, black_box};

#[bench]
fn bench_positive(bencher: &mut Bencher) {
    bencher.iter(|| sum_of_n(black_box(1000)));
}

#[bench]
fn bench_negative(bencher: &mut Bencher) {
    bencher.iter(|| sum_of_n(black_box(-1000)));
}
