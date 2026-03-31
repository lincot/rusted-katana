#![feature(test)]

extern crate test;
use core::array;

use element_equals_its_index::index_equals_value;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let mut arr: [_; if cfg!(miri) { 7 } else { 1024 }] = array::from_fn(|_| rng.random());
    arr.sort_unstable();
    bencher.iter(|| index_equals_value(black_box(&arr)));
}
