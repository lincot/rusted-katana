#![feature(test)]

extern crate test;
use herons_formula_1::heron;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| heron(black_box(3), black_box(4), black_box(5)));
}
