#![feature(test)]

extern crate test;
use core::array;

use dice_rotation::count_min_rotations;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let dice: [_; if cfg!(miri) { 16 } else { 1024 }] = array::from_fn(|_| rng.random_range(1..=6));
    bencher.iter(|| count_min_rotations(black_box(&dice)));
}
