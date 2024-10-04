#![feature(test)]

extern crate test;
use core::array;
use longest_strict_bouncy_subarray::longest_bouncy_list;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; if cfg!(miri) { 60 } else { 1000 }] = array::from_fn(|_| rng.gen_range(-5..=5));
    bencher.iter(|| longest_bouncy_list(black_box(&arr)));
}
