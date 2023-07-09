#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::string::String;
use core::array;
use maximum_length_difference::mx_dif_lg;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let [a1, a2]: [[_; 100]; 2] = array::from_fn(|_| {
        array::from_fn(|_| {
            let len = rng.gen_range(0..1000);
            let mut s = String::with_capacity(len);
            unsafe { s.as_mut_vec().set_len(len) };
            s
        })
    });
    let a1: [_; 100] = array::from_fn(|i| a1[i].as_str());
    let a2: [_; 100] = array::from_fn(|i| a2[i].as_str());
    bencher.iter(|| mx_dif_lg(black_box(a1.into()), black_box(a2.into())));
}
