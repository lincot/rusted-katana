#![feature(test)]

extern crate test;
use core::array;
use product_array_array_series_number_5::product_array;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; 10] = array::from_fn(|_| rng.random_range(1..100));
    bencher.iter(|| product_array(black_box(&arr)));
}
