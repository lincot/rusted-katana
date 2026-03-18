#![feature(test)]

extern crate test;
use core::array;
use dominant_array_elements::solve;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; if cfg!(miri) { 16 } else { 1024 }] = array::from_fn(|_| rng.random());
    bencher.iter(|| solve(black_box(&arr)));
}
