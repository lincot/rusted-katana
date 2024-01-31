#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64;
use simple_letter_removal::solve;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let s: [_; 1000] = array::from_fn(|_| rng.gen_range(b'a'..=b'z'));
    bencher.iter(|| {
        solve(
            black_box(unsafe { core::str::from_utf8_unchecked(&s) }),
            black_box(s.len() / 2),
        )
    });
}
