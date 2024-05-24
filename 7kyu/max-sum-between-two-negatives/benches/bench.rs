#![feature(test)]

extern crate test;
use core::array;
use max_sum_between_two_negatives::max_sum_between_two_negatives;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let nums: [_; 100] = array::from_fn(|_| rng.gen_range(-20..20));
    bencher.iter(|| max_sum_between_two_negatives(black_box(&nums)));
}
