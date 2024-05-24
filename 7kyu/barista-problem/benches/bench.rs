#![feature(test)]

extern crate test;
use barista_problem::barista;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let coffees: [_; 10] = array::from_fn(|_| rng.gen());
    bencher.iter(|| barista(black_box(&coffees)));
}
