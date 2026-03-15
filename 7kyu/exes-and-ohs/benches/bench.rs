#![feature(test)]

extern crate test;
use core::array;
use exes_and_ohs::xo;
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let string: [_; if cfg!(miri) { 8 } else { 1024 }] =
        array::from_fn(|_| *[b'o', b'O', b'x', b'X', b'm'].choose(&mut rng).unwrap());
    let string = unsafe { core::str::from_utf8_unchecked(&string) };
    bencher.iter(|| xo(black_box(string)));
}
