#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};
use unique_in_order::unique_in_order;

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let sequence: [_; 1024] = array::from_fn(|_| rng.gen_range(0..3));
    bencher.iter(|| unique_in_order(black_box(sequence)));
}
