#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use spin_around_touch_the_ground::spin_around;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let lst: [_; if cfg!(miri) { 60 } else { 1000 }] =
        array::from_fn(|_| if rng.gen() { "left" } else { "right" });
    bencher.iter(|| spin_around(black_box(&lst)));
}
