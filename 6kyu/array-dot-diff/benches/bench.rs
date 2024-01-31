#![feature(test)]

extern crate test;
use array_dot_diff::array_diff;
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
    let [a, b]: [[_; 100]; 2] = array::from_fn(|_| array::from_fn(|_| rng.gen_range(-50..50)));
    bencher.iter(|| array_diff(black_box(a.into()), black_box(b.into())));
}
