#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use selecting_quotients_from_an_array::select_quotients;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; if cfg!(miri) { 64 } else { 1024 }] = array::from_fn(|_| rng.gen_range(0..1000));
    bencher.iter(|| select_quotients(black_box(&arr), black_box(5), black_box("odd")));
}
