#![feature(test)]

extern crate test;
use steps_in_primes::step;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| step(black_box(8), black_box(30_000), black_box(100_000)));
}
