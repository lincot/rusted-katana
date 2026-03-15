#![feature(test)]

extern crate test;
use bit_counting::count_bits;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| count_bits(black_box(12_525_589)));
}
