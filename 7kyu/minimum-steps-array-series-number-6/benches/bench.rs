#![feature(test)]

extern crate test;
use core::array;
use minimum_steps_array_series_number_6::minimum_steps;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let nums: [_; 300] = array::from_fn(|_| rng.gen_range(0..1000));
    let value = nums.iter().sum::<i32>() / 2;
    bencher.iter(|| minimum_steps(black_box(&nums), black_box(value)));
}
