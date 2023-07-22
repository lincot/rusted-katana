#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::string::String;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};
use word_values::word_value;

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let words: [_; 100] = array::from_fn(|_| {
        let len = rng.gen_range(0..100);
        let mut s = String::with_capacity(len);
        unsafe { s.as_mut_vec().set_len(len) };
        for p in &mut *unsafe { s.as_mut_vec() } {
            *p = rng.gen_range(b'a'..=b'z');
        }
        s
    });
    let words: [_; 100] = array::from_fn(|i| words[i].as_str());
    bencher.iter(|| word_value(black_box(&words)));
}
