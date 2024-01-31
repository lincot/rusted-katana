#![feature(test)]

extern crate test;
use corner_circle::corner_circle;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(corner_circle(black_box(17.)));
        }
    });
}
