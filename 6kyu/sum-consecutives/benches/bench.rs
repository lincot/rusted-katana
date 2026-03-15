#![feature(test)]

extern crate test;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use sum_consecutives::sum_consecutives;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    // most upvoted solution needs Vec
    let numbers: Vec<_> = (0..if cfg!(miri) { 32 } else { 1024 })
        .map(|_| rng.random_range(-10..10))
        .collect();
    bencher.iter(|| sum_consecutives(black_box(&numbers)));
}
