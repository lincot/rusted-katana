#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};
use tracking_sums_in_a_process::track_sum;

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let arr: [_; if cfg!(miri) { 32 } else { 1024 }] =
        array::from_fn(|_| rng.gen_range(-1000..1000));
    bencher.iter(|| track_sum(black_box(&arr)));
}
