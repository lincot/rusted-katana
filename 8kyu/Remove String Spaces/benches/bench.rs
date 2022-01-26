#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const X: &str = "8 j 8   mBliB8g  imjB8B8  jl  B";

#[bench]
fn bench(b: &mut Bencher) {
    let x = black_box(X);

    b.iter(|| solution::no_space(x.into()))
}
