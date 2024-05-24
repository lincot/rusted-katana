#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use sort_array_by_string_length::sort_by_length;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; 300] = array::from_fn(|_| {
        let len = rng.gen_range(0..1000);
        let mut s = String::with_capacity(len);
        unsafe { s.as_mut_vec().set_len(len) };
        s
    });
    bencher.iter(|| sort_by_length(black_box(&arr)));
}
