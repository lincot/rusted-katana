#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::string::String;
use alphabet_symmetry::solve;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let strings: [_; 1024] = array::from_fn(|_| {
        let len = rng.gen_range(1..100);
        let mut res = String::with_capacity(len);
        unsafe { res.as_mut_vec().set_len(len) };
        for r in &mut *unsafe { res.as_mut_vec() } {
            *r = if rng.gen() { b'a' - b'A' } else { 0 } + rng.gen_range(b'A'..=b'Z');
        }
        res
    });
    bencher.iter(|| solve(black_box(&strings)));
}
