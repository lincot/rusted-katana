#![no_std]
#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64;
use sum_of_intervals::sum_intervals;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let intervals: [_; 1000] = array::from_fn(|_| {
        let start = rng.gen_range(-1_000_000_000..1_000_000_000 - 10_000_000);
        let width = rng.gen_range(1_000_000..10_000_000);
        (start, start + width)
    });
    bencher.iter(|| sum_intervals(black_box(&intervals)));
}
