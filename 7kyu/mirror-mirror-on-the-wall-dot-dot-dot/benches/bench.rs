#![feature(test)]

extern crate test;
use core::array;
use mirror_mirror_on_the_wall_dot_dot_dot::mirror;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let list: [_; if cfg!(miri) { 30 } else { 300 }] = array::from_fn(|_| rng.gen());
    bencher.iter(|| mirror(black_box(&list)));
}
