#![feature(test)]

extern crate test;
use pi_approximation::iter_pi;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| iter_pi(black_box(1e-05)));
}
