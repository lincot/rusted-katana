#![feature(test)]

extern crate test;
use core::array;

use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};
use tracking_sums_in_a_process::track_sum;

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; if cfg!(miri) { 32 } else { 1024 }] =
        array::from_fn(|_| rng.random_range(-1000..1000));
    bencher.iter(|| track_sum(black_box(&arr)));
}
