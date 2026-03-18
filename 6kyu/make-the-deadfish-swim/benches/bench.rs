#![feature(test)]

extern crate test;
use core::array;
use make_the_deadfish_swim::parse;
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let code: [_; if cfg!(miri) { 32 } else { 1024 }] =
        array::from_fn(|_| *b"idsox".choose(&mut rng).unwrap());
    let code = unsafe { core::str::from_utf8_unchecked(&code) };
    bencher.iter(|| parse(black_box(code)));
}
