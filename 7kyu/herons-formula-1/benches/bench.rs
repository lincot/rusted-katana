#![feature(test)]

extern crate test;
use herons_formula_1::heron;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = 3.;
    let b = 4.;
    let c = 5.;
    bencher.iter(|| {
        for _ in 0..100_000 {
            black_box(heron(a, b, c));
        }
    });
}
