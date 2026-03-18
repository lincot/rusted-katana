#![feature(test)]

extern crate test;
use core::array;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use sum_of_positive::positive_sum;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let slice: [_; if cfg!(miri) { 64 } else { 1024 }] =
        array::from_fn(|_| rng.random_range(-1000..1000));
    bencher.iter(|| positive_sum(black_box(&slice)));
}
