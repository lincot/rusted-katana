#![feature(test)]

extern crate test;
use core::array;
use larger_product_or_sum::sum_or_product;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let list: [_; 100] = array::from_fn(|_| rng.gen_range(1..100));
    let n = list.len() / 8;
    bencher.iter(|| sum_or_product(black_box(&list), black_box(n)));
}
