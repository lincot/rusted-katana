#![feature(test)]

extern crate test;
use alphabet_symmetry::solve;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let strings: [_; if cfg!(miri) { 10 } else { 1024 }] = array::from_fn(|_| {
        let res = (0..rng.gen_range(1..if cfg!(miri) { 5 } else { 100 }))
            .map(|_| if rng.gen() { b'a' - b'A' } else { 0 } + rng.gen_range(b'A'..=b'Z'))
            .collect();
        unsafe { String::from_utf8_unchecked(res) }
    });
    bencher.iter(|| solve(black_box(&strings)));
}
