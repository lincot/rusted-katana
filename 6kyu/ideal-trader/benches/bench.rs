#![feature(test)]

extern crate test;
use core::array;
use ideal_trader::ideal_trader;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let prices: [_; 1024] = array::from_fn(|_| rng.gen_range(0f64..10f64));
    bencher.iter(|| ideal_trader(black_box(&prices)));
}
