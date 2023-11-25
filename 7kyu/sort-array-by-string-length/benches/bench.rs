#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::string::String;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64;
use sort_array_by_string_length::sort_by_length;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let arr: [_; 300] = array::from_fn(|_| {
        let len = rng.gen_range(0..1000);
        let mut s = String::with_capacity(len);
        unsafe { s.as_mut_vec().set_len(len) };
        s
    });
    bencher.iter(|| sort_by_length(black_box(&arr)));
}
