#![feature(test)]

extern crate test;
use core::array;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use sorting_by_bits::sort_by_bit;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; if cfg!(miri) { 50 } else { 1000 }] = array::from_fn(|_| rng.random());
    bencher.iter(|| {
        let mut arr = arr;
        sort_by_bit(black_box(&mut arr));
        arr
    });
}
