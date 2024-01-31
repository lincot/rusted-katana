#![feature(test)]

extern crate test;
use no_arithmetic_progressions::sequence;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(sequence(black_box(717_373)));
        }
    });
}
