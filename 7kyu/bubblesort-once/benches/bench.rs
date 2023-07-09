#![no_std]
#![feature(test)]

extern crate test;
use bubblesort_once::bubblesort_once;
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
    let lst: [_; 1024] = array::from_fn(|_| rng.gen_range(1..1000));
    bencher.iter(|| bubblesort_once(black_box(&lst)));
}
