#![no_std]
#![feature(test)]

extern crate test;
use sequence_convergence::convergence;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| convergence(black_box(5000)));
}
