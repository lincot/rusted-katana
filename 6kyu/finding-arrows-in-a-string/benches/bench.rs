#![feature(test)]

extern crate test;
use core::array;
use finding_arrows_in_a_string::arrow_search;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let string: [_; 1024] = array::from_fn(|_| *b".-=<>".choose(&mut rng).unwrap());
    let string = core::str::from_utf8(&string).unwrap();
    bencher.iter(|| arrow_search(black_box(string)));
}
