#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64;
use remove_exclamation_marks::remove_exclamation_marks;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let input: [_; if cfg!(miri) { 16 } else { 1024 }] =
        array::from_fn(|_| if rng.gen() { b'!' } else { b'a' });
    let input = unsafe { core::str::from_utf8_unchecked(&input) };
    bencher.iter(|| remove_exclamation_marks(black_box(input)));
}
