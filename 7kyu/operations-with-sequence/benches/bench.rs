#![feature(test)]

extern crate test;
use core::array;
use operations_with_sequence::calc;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let array: [_; if cfg!(miri) { 60 } else { 1000 }] = array::from_fn(|_| rng.gen_range(0..1000));
    bencher.iter(|| calc(black_box(array.into())));
}
