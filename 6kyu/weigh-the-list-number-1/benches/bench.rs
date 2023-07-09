#![no_std]
#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};
use weigh_the_list_number_1::weigh_the_list;

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let a: [_; 1024] = array::from_fn(|_| rng.gen());
    bencher.iter(|| weigh_the_list(black_box(a.into())));
}
