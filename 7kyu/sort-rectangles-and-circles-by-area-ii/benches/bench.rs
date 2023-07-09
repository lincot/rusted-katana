#![no_std]
#![feature(test)]

extern crate test;
use core::array;
use either::Either;
use rand::Rng;
use rand_pcg::Pcg64;
use sort_rectangles_and_circles_by_area_ii::sort_by_area;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let seq: [_; 10_000] = array::from_fn(|_| {
        if rng.gen() {
            Either::Left((rng.gen_range(0.0..10.0), rng.gen_range(0.0..10.0)))
        } else {
            Either::Right(rng.gen_range(0.0..10.0))
        }
    });
    bencher.iter(|| sort_by_area(black_box(&seq)));
}
