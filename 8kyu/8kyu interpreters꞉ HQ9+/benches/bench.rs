#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const CODE: &str = "9";

#[bench]
fn bench(b: &mut Bencher) {
    let code = black_box(CODE);

    b.iter(|| solution::hq9(code))
}
