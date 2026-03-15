#![feature(test)]

extern crate test;
use core::array;
use longest_vowel_chain::longest_vowel_chain;
use rand::seq::IndexedRandom;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let string: [_; if cfg!(miri) { 16 } else { 1024 }] =
        array::from_fn(|_| *b"abe".choose(&mut rng).unwrap());
    let s = unsafe { core::str::from_utf8_unchecked(&string) };
    bencher.iter(|| longest_vowel_chain(black_box(s)));
}
