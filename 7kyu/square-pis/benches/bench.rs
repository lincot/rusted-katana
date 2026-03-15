#![feature(test)]

extern crate test;
use square_pis::square_pi;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| square_pi(black_box(100)));
}
