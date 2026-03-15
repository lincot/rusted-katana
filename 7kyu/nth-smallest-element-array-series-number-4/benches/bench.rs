#![feature(test)]

extern crate test;
use core::array;
use nth_smallest_element_array_series_number_4::nth_smallest;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let nums: [_; if cfg!(miri) { 16 } else { 1024 }] = array::from_fn(|_| rng.random());
    bencher.iter(|| nth_smallest(black_box(&nums), black_box(nums.len() / 4)));
}
