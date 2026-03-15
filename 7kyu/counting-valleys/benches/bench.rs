#![feature(test)]

extern crate test;
use core::array;
use counting_valleys::counting_valleys;
use rand::seq::IndexedRandom;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let s: [_; if cfg!(miri) { 16 } else { 1024 }] =
        array::from_fn(|_| *b"UFD".choose(&mut rng).unwrap());
    let s = unsafe { core::str::from_utf8_unchecked(&s) };
    bencher.iter(|| counting_valleys(black_box(s)));
}
