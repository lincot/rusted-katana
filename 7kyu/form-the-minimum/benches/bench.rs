#![feature(test)]

extern crate test;
use core::array;
use form_the_minimum::min_value;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let digits: [_; 10] = array::from_fn(|_| rng.random_range(1..=9));
    bencher.iter(|| min_value(black_box(digits.into())));
}
