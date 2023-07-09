#![no_std]
#![feature(test)]

extern crate test;
use core::array;
use enumerable_magic_number_20_cascading_subsets::each_cons;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let arr: [_; 100] = array::from_fn(|_| rng.gen_range(0..=20));
    bencher.iter(|| each_cons(black_box(&arr), black_box(50)));
}
