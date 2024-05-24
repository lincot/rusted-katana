#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use rank_vector::ranks;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let a: [_; 1000] = array::from_fn(|_| rng.gen());
    bencher.iter(|| ranks(black_box(&a)));
}
