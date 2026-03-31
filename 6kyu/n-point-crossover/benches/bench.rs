#![feature(test)]

extern crate test;
use core::array;

use n_point_crossover::crossover;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    const SIZE: usize = 1024;
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let ns: [_; SIZE / 16] = array::from_fn(|_| rng.random_range(0..SIZE));
    let xs: [_; SIZE] = array::from_fn(|_| rng.random());
    let ys: [_; SIZE] = array::from_fn(|_| rng.random());
    bencher.iter(|| crossover(black_box(&ns), black_box(&xs), black_box(&ys)));
}
