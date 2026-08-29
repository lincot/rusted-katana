#![feature(test)]

extern crate test;
use core::array;

use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use simple_fun_number_250_prefix_sums_to_suffix_sums::prefix_sums_to_suffix_sums;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let mut sum = 0;
    let prefix_sums: [_; if cfg!(miri) { 15 } else { 1000 }] = array::from_fn(|_| {
        sum += rng.random_range(-20..20);
        sum
    });
    bencher.iter(|| prefix_sums_to_suffix_sums(black_box(&prefix_sums)));
}
