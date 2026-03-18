#![feature(test)]

extern crate test;
use magnitude::{S, sqr_modulus};
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        sqr_modulus(black_box(S {
            s: "polar".into(),
            xs: [2531, 3261, 2, 3].into(),
        }))
    });
}
