#![feature(test)]

extern crate test;
use core::array;
use jumps_in_a_cycle_number_1::get_jumps;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let cycle: [_; if cfg!(miri) { 8 } else { 1024 }] =
        array::from_fn(|_| rng.random_range(0..512));
    bencher.iter(|| get_jumps(black_box(cycle.into()), black_box(3)));
}
