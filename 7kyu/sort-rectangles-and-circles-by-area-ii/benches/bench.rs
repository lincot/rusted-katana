#![feature(test)]

extern crate test;
use core::array;
use either::Either;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use sort_rectangles_and_circles_by_area_ii::sort_by_area;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let seq: [_; if cfg!(miri) { 100 } else { 10_000 }] = array::from_fn(|_| {
        if rng.gen() {
            Either::Left((rng.gen_range(0.0..10.0), rng.gen_range(0.0..10.0)))
        } else {
            Either::Right(rng.gen_range(0.0..10.0))
        }
    });
    bencher.iter(|| sort_by_area(black_box(&seq)));
}
