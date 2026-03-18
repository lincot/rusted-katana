#![feature(test)]

extern crate test;
use core::array;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};
use two_oldest_ages_1::two_oldest_ages;

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let ages: [_; if cfg!(miri) { 16 } else { 1024 }] = array::from_fn(|_| rng.random());
    bencher.iter(|| two_oldest_ages(black_box(&ages)));
}
