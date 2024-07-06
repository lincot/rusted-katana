#![feature(test)]

extern crate test;
use magnitude::{sqr_modulus, S};
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        sqr_modulus(black_box(S {
            s: "polar".into(),
            xs: [2531, 3261, 2, 3].into(),
        }))
    });
}
