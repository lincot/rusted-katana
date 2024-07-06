#![feature(test)]

extern crate test;
use no_arithmetic_progressions::sequence;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| sequence(black_box(717_373)));
}
