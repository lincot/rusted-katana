#![feature(test)]

extern crate test;
use core::array;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use remove_the_parentheses_difficult_version::remove_parentheses;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let s: [_; if cfg!(miri) { 100 } else { 10000 }] =
        array::from_fn(|_| *b"(a)".choose(&mut rng).unwrap());
    let s = unsafe { core::str::from_utf8_unchecked(&s) };
    bencher.iter(|| remove_parentheses(black_box(s)));
}
