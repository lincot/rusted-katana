#![feature(test)]

extern crate test;
use core::array;
use even_numbers_in_an_array::even_numbers;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let array: [_; if cfg!(miri) { 64 } else { 1024 }] = array::from_fn(|_| rng.gen());
    bencher.iter(|| even_numbers(black_box(&array), black_box(10)));
}
