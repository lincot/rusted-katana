#![feature(test)]

extern crate test;
use pandemia::infected;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(infected(black_box("01000000X000X011X0X")));
        }
    });
}
