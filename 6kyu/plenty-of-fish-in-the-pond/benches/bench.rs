#![feature(test)]

extern crate test;
use core::array;
use plenty_of_fish_in_the_pond::fish;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let shoal: [_; if cfg!(miri) { 64 } else { 1024 }] =
        array::from_fn(|_| rng.gen_range(b'0'..=b'9'));
    let shoal = unsafe { core::str::from_utf8_unchecked(&shoal) };
    bencher.iter(|| fish(black_box(shoal)));
}
