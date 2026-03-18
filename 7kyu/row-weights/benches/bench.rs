#![feature(test)]

extern crate test;
use core::array;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use row_weights::row_weights;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let array: [_; 35] = array::from_fn(|_| rng.random_range(0..1000));
    bencher.iter(|| row_weights(black_box(array.into())));
}
