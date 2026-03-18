#![feature(test)]

extern crate test;
use core::array;
use maximum_product::adjacent_elements_product;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let xs: [_; if cfg!(miri) { 16 } else { 124 }] = array::from_fn(|_| rng.random_range(0..1000));
    bencher.iter(|| adjacent_elements_product(black_box(&xs)));
}
