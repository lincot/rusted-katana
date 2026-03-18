#![feature(test)]

extern crate test;
use deodorant_evaporator::evaporator;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| evaporator(black_box(10.), black_box(10), black_box(10)));
}
