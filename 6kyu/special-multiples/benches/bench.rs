#![feature(test)]

extern crate test;
use special_multiples::count_spec_mult;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| count_spec_mult(black_box(4), black_box(10_000)));
}
