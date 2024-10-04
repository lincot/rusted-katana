#![feature(test)]

extern crate test;
use core::array;
use maximum_triplet_sum_array_series_number_7::max_tri_sum;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let xs: [_; if cfg!(miri) { 64 } else { 1024 }] =
        array::from_fn(|_| rng.gen_range(-1000..1000));
    bencher.iter(|| max_tri_sum(black_box(&xs)));
}
