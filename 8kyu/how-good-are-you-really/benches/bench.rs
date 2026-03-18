#![feature(test)]

extern crate test;
use core::array;
use how_good_are_you_really::better_than_average;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let class_points: [_; 25] = array::from_fn(|_| rng.random_range(0..100));
    bencher.iter(|| better_than_average(black_box(&class_points), black_box(100)));
}
