#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const S: &str = "алиСА";

#[bench]
fn bench(b: &mut Bencher) {
    let s = black_box(S);

    b.iter(|| solution::hello(s))
}
