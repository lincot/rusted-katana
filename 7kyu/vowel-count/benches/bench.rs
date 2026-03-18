#![feature(test)]

extern crate test;
use core::array;
use rand::seq::IteratorRandom;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};
use vowel_count::get_count;

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let string: [_; if cfg!(miri) { 16 } else { 64 }] =
        array::from_fn(|_| (b'a'..=b'z').choose(&mut rng).unwrap());
    let string = unsafe { core::str::from_utf8_unchecked(&string) };
    bencher.iter(|| get_count(black_box(string)));
}
