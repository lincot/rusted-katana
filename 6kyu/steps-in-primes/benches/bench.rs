#![feature(test)]

extern crate test;
use steps_in_primes::step;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let g = black_box(8);
    let m = black_box(30_000);
    let n = black_box(100_000);
    bencher.iter(|| step(g, m, n));
}
