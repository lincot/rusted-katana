#![feature(test)]

extern crate test;
use bubblesort_once::bubblesort_once;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let lst: [_; if cfg!(miri) { 64 } else { 1024 }] = array::from_fn(|_| rng.gen_range(1..1000));
    bencher.iter(|| bubblesort_once(black_box(&lst)));
}
