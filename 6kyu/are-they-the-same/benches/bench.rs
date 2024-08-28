#![feature(test)]

extern crate test;
use are_they_the_same::comp;
use core::array;
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let mut a: [_; if cfg!(miri) { 100 } else { 1024 }] = array::from_fn(|_| rng.gen_range(1..300));
    let mut b = a.map(|x| x * x);
    a.shuffle(&mut rng);
    b.shuffle(&mut rng);
    bencher.iter(|| comp(black_box(a.into()), black_box(b.into())));
}
