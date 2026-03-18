#![feature(test)]

extern crate test;
use core::array;
use max_diff_easy_1::max_diff;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let numbers: [_; if cfg!(miri) { 16 } else { 1024 }] =
        array::from_fn(|_| rng.random_range(i32::MIN / 4..=i32::MAX / 4));
    bencher.iter(|| max_diff(black_box(&numbers)));
}
