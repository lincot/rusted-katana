#![feature(test)]

extern crate test;
use core::array;
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use sum_of_two_lowest_positive_integers::sum_two_smallest_numbers;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let numbers: [_; 64] = array::from_fn(|_| rng.random());
    bencher.iter(|| sum_two_smallest_numbers(black_box(&numbers)));
}
