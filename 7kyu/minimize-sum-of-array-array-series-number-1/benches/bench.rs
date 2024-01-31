#![feature(test)]

extern crate test;
use core::array;
use minimize_sum_of_array_array_series_number_1::min_sum;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let xs: [_; if cfg!(miri) { 100 } else { 10000 }] = array::from_fn(|_| rng.gen_range(0..1000));
    bencher.iter(|| min_sum(black_box(&xs)));
}
