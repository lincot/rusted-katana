#![feature(test)]

extern crate test;
use core::array;
use gangs::gangs;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let divisors: [_; if cfg!(miri) { 10 } else { 100 }] =
        array::from_fn(|_| rng.gen_range(1..100));
    bencher.iter(|| gangs(black_box(&divisors), black_box(89)));
}
