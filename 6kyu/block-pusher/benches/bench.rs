#![feature(test)]

extern crate test;
use block_pusher::block_pushing;
use core::array;
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let lst: [_; if cfg!(miri) { 64 } else { 1024 }] =
        array::from_fn(|_| match rng.gen_range(0..100u8) {
            ..60 => '-',
            60..90 => '#',
            _ => '>',
        });
    bencher.iter(|| block_pushing(black_box(&lst), black_box(16)));
}
