#![feature(test)]

extern crate test;
use core::array;
use counting_sheep_dot_dot_dot::count_sheep;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let sheep: [_; if cfg!(miri) { 16 } else { 1024 }] = array::from_fn(|i| i % 2 == 0);
    bencher.iter(|| count_sheep(black_box(&sheep)));
}
