#![feature(test)]

extern crate test;
use core::array;

use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use sort_array_by_string_length::sort_by_length;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; if cfg!(miri) { 20 } else { 300 }] = array::from_fn(|_| {
        let len = rng.random_range(0..if cfg!(miri) { 200 } else { 1000 });
        let mut s = String::with_capacity(len);
        unsafe { s.as_mut_vec().set_len(len) };
        s
    });
    bencher.iter(|| sort_by_length(black_box(&arr)));
}
