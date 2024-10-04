#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use remove_the_minimum::remove_smallest;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let numbers: [_; if cfg!(miri) { 64 } else { 1024 }] = array::from_fn(|_| rng.gen());
    bencher.iter(|| remove_smallest(black_box(&numbers)));
}
