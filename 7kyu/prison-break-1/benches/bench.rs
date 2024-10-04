#![feature(test)]

extern crate test;
use core::array;
use prison_break_1::freed_prisoners;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let prison: [_; if cfg!(miri) { 64 } else { 1000 }] =
        array::from_fn(|i| if i == 0 { true } else { rng.gen() });
    bencher.iter(|| freed_prisoners(black_box(&prison)));
}
