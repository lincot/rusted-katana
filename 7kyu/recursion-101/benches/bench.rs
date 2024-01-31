#![feature(test)]

extern crate test;
use recursion_101::solve;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(solve(black_box(8_796_203), black_box(7556)));
        }
    });
}
