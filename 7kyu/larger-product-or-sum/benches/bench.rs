#![feature(test)]

extern crate test;
use core::array;
use larger_product_or_sum::sum_or_product;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let list: [_; if cfg!(miri) { 32 } else { 1024 }] =
        array::from_fn(|_| rng.random_range(1..100));
    let n = list.len() / 8;
    bencher.iter(|| sum_or_product(black_box(&list), black_box(n)));
}
