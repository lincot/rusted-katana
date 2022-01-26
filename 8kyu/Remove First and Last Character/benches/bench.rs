#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const S: &str = "Путин";

#[bench]
fn bench(b: &mut Bencher) {
    let s = black_box(S);

    b.iter(|| solution::remove_char(s))
}
