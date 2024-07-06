#![feature(test)]

extern crate test;
use core::array;
use incrementer::incrementer;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let nums: [_; 1024] = array::from_fn(|_| rng.gen_range(1..1000));
    bencher.iter(|| incrementer(black_box(&nums)));
}
