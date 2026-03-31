#![feature(test)]

extern crate test;
use core::array;

use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use sum_of_intervals::sum_intervals;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let intervals: [_; if cfg!(miri) { 100 } else { 1000 }] = array::from_fn(|_| {
        let start = rng.random_range(-1_000_000_000..1_000_000_000 - 10_000_000);
        let width = rng.random_range(1_000_000..10_000_000);
        (start, start + width)
    });
    bencher.iter(|| sum_intervals(black_box(&intervals)));
}
