#![feature(test)]

extern crate test;
use core::array;
use product_of_maximums_of_array_array_series_number_2::max_product;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let lst: [_; 100] = array::from_fn(|_| rng.gen_range(-5..5));
    bencher.iter(|| max_product(black_box(lst.into()), black_box(10)));
}
