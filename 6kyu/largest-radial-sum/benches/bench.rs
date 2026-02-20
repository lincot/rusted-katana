#![feature(test)]

extern crate test;
use core::array;
use largest_radial_sum::largest_radial_sum;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; if cfg!(miri) { 64 } else { 1024 }] =
        array::from_fn(|_| rng.random_range(-10_000..10_000));
    bencher.iter(|| largest_radial_sum(black_box(&arr), black_box(10)));
}
