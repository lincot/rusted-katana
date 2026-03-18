#![feature(test)]

extern crate test;
use core::array;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use sort_out_the_men_from_boys_1::men_from_boys;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let xs: [_; if cfg!(miri) { 50 } else { 1000 }] =
        array::from_fn(|_| rng.random_range(-1000..1000));
    bencher.iter(|| men_from_boys(black_box(&xs)));
}
