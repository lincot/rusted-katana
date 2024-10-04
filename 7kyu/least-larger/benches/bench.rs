#![feature(test)]

extern crate test;
use core::array;
use least_larger::least_larger;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let a: [_; if cfg!(miri) { 64 } else { 1024 }] = array::from_fn(|_| rng.gen_range(-1000..1000));
    bencher.iter(|| least_larger(black_box(&a), black_box(if cfg!(miri) { 15 } else { 160 })));
}
