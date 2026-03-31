#![feature(test)]

extern crate test;
use core::array;

use elevator_distance::elevator_distance;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let floors: [_; 64] = array::from_fn(|_| rng.random_range(0..30));
    bencher.iter(|| elevator_distance(black_box(&floors)));
}
