#![feature(test)]

extern crate test;
use core::array;
use largest_elements::largest;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let xs: [_; if cfg!(miri) { 64 } else { 1000 }] = array::from_fn(|_| rng.gen());
    bencher.iter(|| largest(black_box(if cfg!(miri) { 10 } else { 300 }), black_box(&xs)));
}
