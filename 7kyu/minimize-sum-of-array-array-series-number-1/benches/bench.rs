#![feature(test)]

extern crate test;
use core::array;
use minimize_sum_of_array_array_series_number_1::min_sum;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let xs: [_; if cfg!(miri) { 100 } else { 10000 }] = array::from_fn(|_| rng.gen_range(0..1000));
    bencher.iter(|| min_sum(black_box(&xs)));
}
