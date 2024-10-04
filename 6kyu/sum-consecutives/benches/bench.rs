#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use sum_consecutives::sum_consecutives;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let numbers: [_; if cfg!(miri) { 32 } else { 1024 }] =
        array::from_fn(|_| rng.gen_range(-10..10));
    bencher.iter(|| sum_consecutives(black_box(&numbers)));
}
