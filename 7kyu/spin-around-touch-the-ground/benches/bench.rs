#![no_std]
#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64;
use spin_around_touch_the_ground::spin_around;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let lst: [_; 1000] = array::from_fn(|_| if rng.gen() { "left" } else { "right" });
    bencher.iter(|| spin_around(black_box(&lst)));
}
