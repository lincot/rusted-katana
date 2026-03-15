#![feature(test)]

extern crate test;
use array_element_parity::solve;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    // most upvoted solution needs Vec
    let arr: Vec<_> = (0..if cfg!(miri) { 64 } else { 1024 })
        .map(|_| rng.random_range(-1000..1500))
        .collect();
    bencher.iter(|| solve(black_box(&arr)));
}
