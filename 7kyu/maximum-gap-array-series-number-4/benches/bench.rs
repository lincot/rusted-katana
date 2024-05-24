#![feature(test)]

extern crate test;
use core::array;
use maximum_gap_array_series_number_4::max_gap;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let xs: [_; 300] = array::from_fn(|_| rng.gen());
    bencher.iter(|| max_gap(black_box(&xs)));
}
