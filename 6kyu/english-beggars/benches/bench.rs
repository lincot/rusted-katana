#![feature(test)]

extern crate test;
use core::array;
use english_beggars::beggars;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let values: [_; if cfg!(miri) { 10 } else { 100 }] = array::from_fn(|_| rng.gen_range(0..1000));
    bencher.iter(|| beggars(black_box(&values), black_box(7)));
}
