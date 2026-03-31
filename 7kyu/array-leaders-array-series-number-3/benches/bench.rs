#![feature(test)]

extern crate test;
use core::array;

use array_leaders_array_series_number_3::array_leaders;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; 100] = array::from_fn(|_| rng.random_range(-1000..1000));
    bencher.iter(|| array_leaders(black_box(&arr)));
}
