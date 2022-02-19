#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const S: &str = "Путин";

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box(S);

    bencher.iter(|| solution::remove_char(s))
}
