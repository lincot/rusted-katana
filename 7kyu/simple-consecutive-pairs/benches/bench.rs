#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use simple_consecutive_pairs::pairs;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; 1024] = array::from_fn(|_| rng.gen_range(-10..10));
    bencher.iter(|| pairs(black_box(&arr)));
}
