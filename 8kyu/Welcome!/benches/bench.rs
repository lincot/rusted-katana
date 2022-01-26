#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const LANGUAGE: &str = "swedish";

#[bench]
fn bench(b: &mut Bencher) {
    let language = black_box(LANGUAGE);

    b.iter(|| solution::greet(language))
}
