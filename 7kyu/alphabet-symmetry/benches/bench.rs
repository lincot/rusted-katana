#![feature(test)]

extern crate test;
use alphabet_symmetry::solve;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let strings: [_; if cfg!(miri) { 10 } else { 1024 }] = array::from_fn(|_| {
        let res = (0..rng.gen_range(1..100))
            .map(|_| if rng.gen() { b'a' - b'A' } else { 0 } + rng.gen_range(b'A'..=b'Z'))
            .collect();
        unsafe { String::from_utf8_unchecked(res) }
    });
    bencher.iter(|| solve(black_box(&strings)));
}
