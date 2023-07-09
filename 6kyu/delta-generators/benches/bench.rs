#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::vec::Vec;
use core::array;
use delta_generators::delta;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let values: [_; 1024] = array::from_fn(|_| rng.gen_range(-100..100));
    let level = 50;
    bencher.iter(|| delta(black_box(values), black_box(level)).collect::<Vec<_>>());
}
