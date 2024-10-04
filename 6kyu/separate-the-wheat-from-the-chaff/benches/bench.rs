#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use separate_the_wheat_from_the_chaff::wheat_from_chaff;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let xs: [_; if cfg!(miri) { 64 } else { 1024 }] =
        array::from_fn(|_| rng.gen_range(-1000..1000));
    bencher.iter(|| wheat_from_chaff(black_box(&xs)));
}
